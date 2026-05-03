# Set the minimum version of CMake that can be used
cmake_minimum_required(VERSION 3.10)

# Set the project name
project (%%TEST_NAME%%)

# Stop fetch content warning
if (CMAKE_VERSION VERSION_GREATER_EQUAL "3.30.0")
  cmake_policy(SET CMP0169 OLD)
endif()

# Set the c++ standard
set(CMAKE_CXX_STANDARD 17)

set(INCLUDE_YAML %%INCLUDE_YAML%%)

# Add the test executable
file(GLOB GeneratedSRC
  "*.cpp"
)

add_executable(full_example
  ../%%TEST_NAME%%_test.cpp
  ${GeneratedSRC}
)

# Add the hpp files
target_include_directories(%%TEST_NAME%% PRIVATE . .. %%TERMITE_PATH%%)

# Add yaml-cpp
if(INCLUDE_YAML)
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

target_sources(%%TEST_NAME%% PRIVATE %%TERMITE_PATH%%/termite-yaml.cpp)
target_link_libraries(%%TEST_NAME%% PUBLIC yaml-cpp::yaml-cpp)
endif()