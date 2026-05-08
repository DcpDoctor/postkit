cmake_minimum_required(VERSION 3.25)
project(postkit VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Options
option(BUILD_TESTING "Build tests" ON)
option(BUILD_PYTHON_BINDINGS "Build Python/SWIG bindings" OFF)

if(BUILD_PYTHON_BINDINGS)
  set(CMAKE_POSITION_INDEPENDENT_CODE ON)
endif()

# Dependencies
if(WIN32)
  find_package(LibXml2 REQUIRED)
  set(LIBXML2_INCLUDE_DIRS ${LIBXML2_INCLUDE_DIR})
  set(LIBXML2_LIBRARIES LibXml2::LibXml2)
else()
  find_package(PkgConfig REQUIRED)
  pkg_check_modules(LIBXML2 REQUIRED libxml-2.0)
endif()
find_package(OpenSSL REQUIRED)

# Optional: spdlog (bundled or system)
if(NOT TARGET spdlog::spdlog)
  if(EXISTS "${CMAKE_CURRENT_SOURCE_DIR}/extern/spdlog/CMakeLists.txt")
    add_subdirectory(extern/spdlog EXCLUDE_FROM_ALL)
  else()
    find_package(spdlog REQUIRED)
  endif()
endif()

# Core library
add_library(postkit_core STATIC
    src/encode.cpp
    src/transcode.cpp
    src/hash.cpp
    src/mxf_wrap.cpp
    src/colour.cpp
    src/loudness.cpp
    src/atmos.cpp
    src/job_queue.cpp
    src/preferences.cpp
    src/rest_api.cpp
    src/profiles.cpp
    src/burnin.cpp
    src/report.cpp
    src/watch.cpp
    src/shell_completion.cpp
    src/portable.cpp
    src/preview.cpp
    src/ingest.cpp
    src/conform.cpp
    src/metadata_edit.cpp
    src/certificate.cpp
    src/dolby_vision.cpp
    src/dashboard.cpp
    src/watermark.cpp
    src/dcdm.cpp
    src/version_tracker.cpp
    src/trailer.cpp
    src/accessibility.cpp
)

target_include_directories(postkit_core PUBLIC
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>
)

target_include_directories(postkit_core PRIVATE
    ${LIBXML2_INCLUDE_DIRS}
)

target_link_libraries(postkit_core PUBLIC
    spdlog::spdlog
    OpenSSL::SSL
    OpenSSL::Crypto
    ${LIBXML2_LIBRARIES}
)

# Tests
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME AND BUILD_TESTING)
  enable_testing()
  add_executable(postkit_test tests/test_main.cpp)
  target_link_libraries(postkit_test PRIVATE postkit_core)
  add_test(NAME postkit_test COMMAND postkit_test)
endif()

# Python bindings
if(BUILD_PYTHON_BINDINGS)
  add_subdirectory(bindings/python)
endif()
