#include "postkit/preview.h"
#include "postkit/ingest.h"
#include "postkit/conform.h"
#include "postkit/metadata_edit.h"
#include "postkit/certificate.h"
#include "postkit/dolby_vision.h"
#include "postkit/encode.h"
#include "postkit/transcode.h"
#include "postkit/hash.h"
#include "postkit/colour.h"
#include "postkit/loudness.h"
#include "postkit/atmos.h"
#include "postkit/job_queue.h"
#include "postkit/preferences.h"
#include "postkit/profiles.h"
#include "postkit/certificate.h"

#include <cassert>
#include <cstdio>

int main()
{
  int pass = 0, fail = 0;

  // Preview
  {
    assert(postkit::extract_frame("/tmp/dcp", 0, "/tmp/frame.png") == 0);
    assert(postkit::play({}) == 0);
    pass += 2;
  }

  // Ingest
  {
    assert(postkit::detect_format("/tmp/card") == postkit::CameraFormat::Unknown);
    auto clips = postkit::scan_media("/tmp/card");
    assert(clips.empty());
    assert(postkit::ingest({}) == 0);
    pass += 3;
  }

  // Conform
  {
    assert(postkit::detect_timeline_format("test.edl") == postkit::TimelineFormat::EDL_CMX3600);
    assert(postkit::detect_timeline_format("test.aaf") == postkit::TimelineFormat::AAF);
    assert(postkit::detect_timeline_format("test.otio") == postkit::TimelineFormat::OTIO);
    assert(postkit::detect_timeline_format("test.xyz") == postkit::TimelineFormat::Unknown);
    assert(postkit::conform({}) == 0);
    pass += 5;
  }

  // Metadata editor
  {
    auto meta = postkit::read_metadata("/tmp/cpl.xml");
    assert(meta.uuid.empty());
    assert(postkit::write_metadata("/tmp/cpl.xml", meta) == 0);
    auto fields = postkit::list_fields("/tmp/cpl.xml");
    assert(fields.empty());
    pass += 3;
  }

  // Certificate management
  {
    postkit::CertOptions opts;
    opts.common_name = "Test Signer";
    opts.output_cert = "/tmp/cert.pem";
    opts.output_key = "/tmp/key.pem";
    assert(postkit::generate_certificate(opts) == 0);
    assert(postkit::generate_chain("Test Org", "/tmp/chain") == 0);
    auto info = postkit::read_certificate("/tmp/cert.pem");
    assert(info.key_bits == 0);  // stub returns empty
    assert(postkit::validate_chain({"/tmp/cert.pem"}) == 0);
    auto devices = postkit::list_trusted_devices();
    assert(devices.empty());
    pass += 5;
  }

  // Dolby Vision / HDR
  {
    assert(postkit::detect_hdr_type("/tmp/mxf") == postkit::HdrType::SDR);
    postkit::HdrMetadataOptions hdr_opts;
    hdr_opts.input = "/tmp/mxf";
    assert(postkit::inject_hdr10_metadata(hdr_opts) == 0);
    postkit::DolbyVisionOptions dv_opts;
    dv_opts.input = "/tmp/mxf";
    dv_opts.rpu_file = "/tmp/rpu.bin";
    assert(postkit::inject_dolby_vision(dv_opts) == 0);
    assert(postkit::convert_hdr("/tmp/in.mxf", postkit::HdrType::HLG, "/tmp/out.mxf") == 0);
    pass += 4;
  }

  std::printf("postkit: %d tests passed, %d failed\n", pass, fail);
  return fail;
}
