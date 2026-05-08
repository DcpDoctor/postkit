# postkit

Shared library for DCP Wizard, IMF Wizard, and DCP Doctor — common post-production functionality.

## Modules

| Module | Purpose |
|--------|---------|
| `encode` | JPEG 2000 encoding (grok / OpenJPEG) |
| `transcode` | Video format conversion (via ffmpeg) |
| `hash` | SHA-1 / SHA-256 file hashing |
| `mxf_wrap` | MXF track file wrapping |
| `colour` | Colour space conversion (Rec.709, P3, XYZ) |
| `loudness` | Audio loudness measurement (EBU R128) |
| `atmos` | Dolby Atmos IAB packaging |
| `job_queue` | Background job scheduling |
| `preferences` | JSON preferences (XDG/AppData) |
| `rest_api` | HTTP REST API server |
| `profiles` | Delivery profile presets |
| `burnin` | Subtitle/watermark burn-in |
| `report` | HTML/JSON QC report generation |
| `watch` | Watch folder automation |
| `shell_completion` | Bash/Zsh/Fish completion |
| `portable` | Portable/USB deployment |
| `preview` | Frame-accurate DCP/IMF playback and frame extraction |
| `ingest` | Camera raw ingest (ARRI, RED, Sony, Canon, BRAW) |
| `conform` | EDL/AAF/XML timeline import and reel assembly |
| `metadata_edit` | CPL/OPL metadata editor |
| `certificate` | X.509 certificate generation and trust management |
| `dolby_vision` | Dolby Vision RPU, HDR10, HLG metadata handling |
| `dashboard` | Real-time job monitoring and analytics dashboard |
| `watermark` | Forensic watermarking (NexGuard, Civolution, internal) |
| `dcdm` | Digital Cinema Distribution Master creation and export |
| `version_tracker` | Content versioning database (delivery history) |
| `trailer` | Theatrical trailer packaging (ratings cards, leaders) |
| `accessibility` | Accessibility compliance checking (CVAA, EAA, AODA, Ofcom) |

## Building

```bash
git clone --recurse-submodules https://github.com/DcpDoctor/postkit.git
cd postkit
cmake -B build -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel
ctest --test-dir build --output-on-failure
```

## Usage

Link against `postkit_core` in your CMakeLists.txt:

```cmake
add_subdirectory(extern/postkit EXCLUDE_FROM_ALL)
target_link_libraries(myapp PRIVATE postkit_core)
```

## License

GPL-3.0
