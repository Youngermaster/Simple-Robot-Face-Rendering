cmake_minimum_required(VERSION 3.20)
project(RobotFaceComparison)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_C_STANDARD 11)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Build options
option(BUILD_RAYLIB "Build Raylib implementation" ON)
option(BUILD_SKIA "Build Skia implementation" ON)

# Find required packages
find_package(SDL2 REQUIRED)

if(BUILD_RAYLIB)
    message(STATUS "Building Raylib implementation")
    add_subdirectory(raylib)
endif()

if(BUILD_SKIA)
    message(STATUS "Building Skia implementation")
    add_subdirectory(skia)
endif()
