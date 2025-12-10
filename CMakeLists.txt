cmake_minimum_required(VERSION 3.15)
project(cppsim VERSION 0.1.0 LANGUAGES CXX)

# Set C++17 standard
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Include FetchContent module  
include(FetchContent)

# =============================================================================
# Dependency Management
# =============================================================================

# Boost - Use system-installed for faster builds (Boost 1.83+ installed)
message(STATUS "Finding Boost...")
find_package(Boost 1.82 REQUIRED COMPONENTS system)
if(NOT TARGET Boost::beast)
  # Beast is header-only, create interface target
  add_library(Boost::beast INTERFACE IMPORTED)
  target_include_directories(Boost::beast INTERFACE ${Boost_INCLUDE_DIRS})
endif()
if(NOT TARGET Boost::asio)
  # Asio is header-only, create interface target  
  add_library(Boost::asio INTERFACE IMPORTED)
  target_include_directories(Boost::asio INTERFACE ${Boost_INCLUDE_DIRS})
endif()

# nlohmann/json
message(STATUS "Fetching nlohmann/json...")
FetchContent_Declare(
  json
  GIT_REPOSITORY https://github.com/nlohmann/json.git
  GIT_TAG v3.11.2
)
FetchContent_MakeAvailable(json)

# Google Test
message(STATUS "Fetching Google Test...")
FetchContent_Declare(
  googletest
  GIT_REPOSITORY https://github.com/google/googletest.git
  GIT_TAG v1.14.0
)
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

# =============================================================================
# Subdirectories
# =============================================================================

add_subdirectory(src/common)
add_subdirectory(src/server)
add_subdirectory(src/client)
add_subdirectory(tests)

# =============================================================================
# Summary
# =============================================================================

message(STATUS "")
message(STATUS "===================================")
message(STATUS "cppsim - C++ Poker Server Simulation")
message(STATUS "===================================")
message(STATUS "Build type: ${CMAKE_BUILD_TYPE}")
message(STATUS "C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "Boost Version: ${Boost_VERSION}")
message(STATUS "Targets:")
message(STATUS "  - poker_server (server executable)")
message(STATUS "  - poker_client (client executable)")
message(STATUS "  - poker_common (shared library)")
message(STATUS "  - poker_tests (test executable)")
message(STATUS "===================================")
message(STATUS "")
