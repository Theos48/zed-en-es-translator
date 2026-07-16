cmake_minimum_required(VERSION 3.20)
project(translator_embedded_runtime LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

if(CMAKE_CXX_FLAGS MATCHES "(^|[ ;])-march=native($|[ ;])")
  message(FATAL_ERROR "The embedded runtime must use the reviewed portable CPU baseline")
endif()

option(
  TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE
  "Build only the synthetic wire-contract fixture (never a production provider)"
  OFF
)

add_executable(translator-embedded-runtime src/main.cpp)

if(TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE)
  target_compile_definitions(
    translator-embedded-runtime
    PRIVATE TRANSLATOR_EMBEDDED_CONTROLLED_FIXTURE=1
  )
else()
  set(
    TRANSLATOR_BERGAMOT_SOURCE_DIR
    ""
    CACHE PATH
    "Exact locked Mozilla translations source checkout"
  )
  if(NOT EXISTS "${TRANSLATOR_BERGAMOT_SOURCE_DIR}/inference/CMakeLists.txt")
    message(FATAL_ERROR "The exact locked Bergamot source checkout is required")
  endif()

  set(BUILD_ARCH x86-64 CACHE STRING "Portable embedded CPU baseline" FORCE)
  set(GIT_SUBMODULE OFF CACHE BOOL "Source was fetched and verified separately" FORCE)
  set(COMPILE_TESTS OFF CACHE BOOL "Do not build upstream tests" FORCE)
  set(COMPILE_UNIT_TESTS OFF CACHE BOOL "Do not build upstream unit tests" FORCE)
  set(COMPILE_EXAMPLES OFF CACHE BOOL "Do not build upstream examples" FORCE)
  set(COMPILE_SERVER OFF CACHE BOOL "The embedded provider is never a service" FORCE)
  add_subdirectory(
    "${TRANSLATOR_BERGAMOT_SOURCE_DIR}/inference"
    "${CMAKE_CURRENT_BINARY_DIR}/bergamot"
    EXCLUDE_FROM_ALL
  )
  target_link_libraries(
    translator-embedded-runtime
    PRIVATE bergamot-translator-source
  )
  target_include_directories(
    translator-embedded-runtime
    PRIVATE
      "${TRANSLATOR_BERGAMOT_SOURCE_DIR}/inference/src"
      "${TRANSLATOR_BERGAMOT_SOURCE_DIR}/inference"
  )
endif()

target_compile_options(
  translator-embedded-runtime
  PRIVATE -Wall -Wextra -Wpedantic -Werror -O2 -march=x86-64 -mtune=generic
)
