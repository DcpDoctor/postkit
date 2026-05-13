use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Job type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    Transcode,
    Encode,
    Create,
    Validate,
    Loudness,
    Qc,
    Copy,
    Kdm,
}

/// Job state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// A queued job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: u64,
    pub job_type: JobType,
    pub state: JobState,
    pub priority: i32,
    pub description: String,
    pub input: PathBuf,
    pub output: PathBuf,
    pub progress: f64,
    pub error: String,
    /// Job IDs that must complete before this job can run.
    pub depends_on: Vec<u64>,
}

impl Default for Job {
    fn default() -> Self {
        Self {
            id: 0,
            job_type: JobType::Create,
            state: JobState::Queued,
            priority: 0,
            description: String::new(),
            input: PathBuf::new(),
            output: PathBuf::new(),
            progress: 0.0,
            error: String::new(),
            depends_on: Vec::new(),
        }
    }
}

/// In-process job queue with priority and dependency support.
#[derive(Debug, Clone)]
pub struct JobQueue {
    inner: Arc<Mutex<JobQueueInner>>,
}

#[derive(Debug)]
struct JobQueueInner {
    jobs: VecDeque<Job>,
    next_id: u64,
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(JobQueueInner {
                jobs: VecDeque::new(),
                next_id: 1,
            })),
        }
    }

    /// Submit a new job. Returns the assigned job ID.
    pub fn submit(&self, mut job: Job) -> u64 {
        let mut inner = self.inner.lock().unwrap();
        job.id = inner.next_id;
        job.state = JobState::Queued;
        inner.next_id += 1;
        let id = job.id;
        inner.jobs.push_back(job);
        id
    }

    /// Cancel a job by ID.
    pub fn cancel(&self, id: u64) -> bool {
        let mut inner = self.inner.lock().unwrap();
        if let Some(job) = inner.jobs.iter_mut().find(|j| j.id == id)
            && (job.state == JobState::Queued || job.state == JobState::Running)
        {
            job.state = JobState::Cancelled;
            return true;
        }
        false
    }

    /// Get a job by ID.
    pub fn get(&self, id: u64) -> Option<Job> {
        let inner = self.inner.lock().unwrap();
        inner.jobs.iter().find(|j| j.id == id).cloned()
    }

    /// List all jobs.
    pub fn list(&self) -> Vec<Job> {
        let inner = self.inner.lock().unwrap();
        inner.jobs.iter().cloned().collect()
    }

    /// Get the next runnable job (queued, all dependencies completed).
    pub fn next_runnable(&self) -> Option<Job> {
        let inner = self.inner.lock().unwrap();
        let completed_ids: Vec<u64> = inner
            .jobs
            .iter()
            .filter(|j| j.state == JobState::Completed)
            .map(|j| j.id)
            .collect();

        inner
            .jobs
            .iter()
            .find(|j| {
                j.state == JobState::Queued
                    && j.depends_on.iter().all(|dep| completed_ids.contains(dep))
            })
            .cloned()
    }

    /// Update job state.
    pub fn set_state(&self, id: u64, state: JobState) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(job) = inner.jobs.iter_mut().find(|j| j.id == id) {
            job.state = state;
        }
    }

    /// Update job progress (0.0 to 1.0).
    pub fn set_progress(&self, id: u64, progress: f64) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(job) = inner.jobs.iter_mut().find(|j| j.id == id) {
            job.progress = progress;
        }
    }

    /// Set error message and mark as failed.
    pub fn fail(&self, id: u64, error: &str) {
        let mut inner = self.inner.lock().unwrap();
        if let Some(job) = inner.jobs.iter_mut().find(|j| j.id == id) {
            job.state = JobState::Failed;
            job.error = error.to_string();
        }
    }
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert job type to display string.
pub fn job_type_to_string(jt: JobType) -> &'static str {
    match jt {
        JobType::Transcode => "Transcode",
        JobType::Encode => "Encode",
        JobType::Create => "Create",
        JobType::Validate => "Validate",
        JobType::Loudness => "Loudness",
        JobType::Qc => "QC",
        JobType::Copy => "Copy",
        JobType::Kdm => "KDM",
    }
}

/// Convert job state to display string.
pub fn job_state_to_string(js: JobState) -> &'static str {
    match js {
        JobState::Queued => "Queued",
        JobState::Running => "Running",
        JobState::Completed => "Completed",
        JobState::Failed => "Failed",
        JobState::Cancelled => "Cancelled",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn submit_and_list() {
        let q = JobQueue::new();
        let id = q.submit(Job {
            job_type: JobType::Encode,
            description: "Encode reel 1".to_string(),
            ..Default::default()
        });
        assert_eq!(id, 1);
        assert_eq!(q.list().len(), 1);
    }

    #[test]
    fn cancel() {
        let q = JobQueue::new();
        let id = q.submit(Job::default());
        assert!(q.cancel(id));
        assert_eq!(q.get(id).unwrap().state, JobState::Cancelled);
    }

    #[test]
    fn dependency_ordering() {
        let q = JobQueue::new();
        let id1 = q.submit(Job {
            job_type: JobType::Encode,
            ..Default::default()
        });
        let _id2 = q.submit(Job {
            job_type: JobType::Create,
            depends_on: vec![id1],
            ..Default::default()
        });

        // Job 2 depends on Job 1, so next_runnable should be Job 1
        let next = q.next_runnable().unwrap();
        assert_eq!(next.id, id1);

        // Complete Job 1
        q.set_state(id1, JobState::Completed);

        // Now Job 2 should be runnable
        let next = q.next_runnable().unwrap();
        assert_eq!(next.id, _id2);
    }
}
