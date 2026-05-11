use serde::{Deserialize, Serialize};

/// Accessibility standard to check against.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityStandard {
    /// US: 21st Century Communications and Video Accessibility Act
    Cvaa,
    /// EU: European Accessibility Act (2025)
    Eaa,
    /// Canada: Accessibility for Ontarians with Disabilities Act
    Aoda,
    /// UK: Ofcom broadcasting accessibility code
    Ofcom,
}

/// Accessibility track type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AccessibilityTrack {
    /// AD — visually impaired narration
    AudioDescription,
    /// HI — SDH/CC subtitles for deaf/hard of hearing
    HearingImpaired,
    /// SL — sign language video overlay
    SignLanguage,
    /// OC — burned-in captions
    OpenCaptions,
    /// CC — CEA-608/708 caption stream
    ClosedCaptions,
    /// Director/audio commentary
    Commentary,
}

/// Severity of an accessibility finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Severity {
    Error,
    Warning,
    Info,
}

/// Single compliance finding.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityFinding {
    pub severity: Severity,
    pub track_type: AccessibilityTrack,
    /// e.g. "CVAA-3.1", "EAA-4.2"
    pub rule_id: String,
    pub description: String,
    pub recommendation: String,
}

/// Result of accessibility compliance check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityResult {
    pub compliant: bool,
    pub standard: AccessibilityStandard,
    pub findings: Vec<AccessibilityFinding>,
    pub errors: u32,
    pub warnings: u32,
    pub tracks_present: Vec<AccessibilityTrack>,
    pub tracks_missing: Vec<AccessibilityTrack>,
}

/// Check accessibility compliance of a DCP or IMP.
pub fn check_accessibility(
    _package_dir: &std::path::Path,
    standard: AccessibilityStandard,
) -> AccessibilityResult {
    tracing::warn!("check_accessibility: not yet implemented");
    AccessibilityResult {
        compliant: false,
        standard,
        findings: Vec::new(),
        errors: 0,
        warnings: 0,
        tracks_present: Vec::new(),
        tracks_missing: Vec::new(),
    }
}

/// Check accessibility compliance against multiple standards.
pub fn check_accessibility_multi(
    package_dir: &std::path::Path,
    standards: &[AccessibilityStandard],
) -> Vec<AccessibilityResult> {
    standards
        .iter()
        .map(|&s| check_accessibility(package_dir, s))
        .collect()
}
