# Set the minimum version of CMake that can be used
cmake_minimum_required(VERSION 3.10)

# Set the project name
project (full_example)

# Stop fetch content warning
if (CMAKE_VERSION VERSION_GREATER_EQUAL "3.30.0")
  cmake_policy(SET CMP0169 OLD)
endif()

# Set the c++ standard
set(CMAKE_CXX_STANDARD 17)

# Add an executables
add_executable(full_example full_example_test.cpp full_example.cpp ../../../src/cpp/termite-yaml.cpp)

# Add the hpp files
target_include_directories(full_example PRIVATE ../../../src/cpp)

# Add yaml-cpp
include(FetchContent)

FetchContent_Declare(
  yaml-cpp
  GIT_REPOSITORY https://github.com/jbeder/yaml-cpp.git
  GIT_TAG 8fbf344 # Can be a tag (yaml-cpp-x.x.x), a commit hash, or a branch name (master)
)
FetchContent_GetProperties(yaml-cpp)

if(NOT yaml-cpp_POPULATED)
  message(STATUS "Fetching yaml-cpp...")
  FetchContent_Populate(yaml-cpp)
  add_subdirectory(${yaml-cpp_SOURCE_DIR} ${yaml-cpp_BINARY_DIR})
endif()

target_link_libraries(full_example PUBLIC yaml-cpp::yaml-cpp)
