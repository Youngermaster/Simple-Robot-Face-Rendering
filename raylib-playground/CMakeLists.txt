cmake_minimum_required(VERSION 3.15)
project(RaylibPlayground VERSION 1.0.0)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Build options
option(BUILD_SOLAR_SYSTEM "Build Solar System example" ON)
option(BUILD_UI_COMPONENTS "Build UI Components example" ON)
option(BUILD_PARTICLE_SYSTEM "Build Particle System example" ON)
option(BUILD_BEZIER_CURVES "Build Bezier Curves example" ON)

# Add subdirectories
if(BUILD_SOLAR_SYSTEM)
    add_subdirectory(01-solar-system)
endif()

if(BUILD_UI_COMPONENTS)
    add_subdirectory(02-ui-components)
endif()

if(BUILD_PARTICLE_SYSTEM)
    add_subdirectory(03-particle-system)
endif()

if(BUILD_BEZIER_CURVES)
    add_subdirectory(04-bezier-curves)
endif()

# Print configuration summary
message(STATUS "")
message(STATUS "Raylib Playground Configuration:")
message(STATUS "  Build Solar System: ${BUILD_SOLAR_SYSTEM}")
message(STATUS "  Build UI Components: ${BUILD_UI_COMPONENTS}")
message(STATUS "  Build Particle System: ${BUILD_PARTICLE_SYSTEM}")
message(STATUS "  Build Bezier Curves: ${BUILD_BEZIER_CURVES}")
message(STATUS "")
