cmake_minimum_required(VERSION 3.15)
project(RobotFaceRaylib VERSION 1.0.0 LANGUAGES C CXX)

# Set C and C++ standards
set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Raylib
find_package(raylib QUIET)

if (NOT raylib_FOUND)
    # Try pkg-config as fallback
    find_package(PkgConfig)
    if (PkgConfig_FOUND)
        pkg_check_modules(RAYLIB QUIET raylib)
    endif()

    if (NOT RAYLIB_FOUND)
        message(STATUS "Raylib not found via CMake or pkg-config, trying manual paths...")
        # Common installation paths for macOS (Homebrew)
        set(RAYLIB_INCLUDE_DIRS "/opt/homebrew/include")
        set(RAYLIB_LIBRARY_DIRS "/opt/homebrew/lib")
        set(RAYLIB_LIBRARIES "raylib")
    endif()
endif()

# Include directories
include_directories(
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${RAYLIB_INCLUDE_DIRS}
)

# Link directories
link_directories(${RAYLIB_LIBRARY_DIRS})

# Build options
option(BUILD_ORIGINAL "Build the original monolithic version" ON)
option(BUILD_C_MODULAR "Build the modular C version" ON)
option(BUILD_CPP_MODERN "Build the modern C++ version" ON)

# ============================================================================
# Original Version - Monolithic C
# ============================================================================
if(BUILD_ORIGINAL)
    message(STATUS "Building original monolithic C version")

    add_executable(robot_face_raylib
        src/robot_face_raylib.c
    )

    target_include_directories(robot_face_raylib PRIVATE
        ${CMAKE_SOURCE_DIR}/common
    )

    target_link_libraries(robot_face_raylib
        ${RAYLIB_LIBRARIES}
        m  # Math library
    )

    # Platform-specific settings
    if(APPLE)
        target_link_libraries(robot_face_raylib
            "-framework IOKit"
            "-framework Cocoa"
            "-framework OpenGL"
        )
    elseif(UNIX)
        target_link_libraries(robot_face_raylib
            GL
            pthread
            dl
            rt
            X11
        )
    endif()

    # Copy to root build directory for easy access
    set_target_properties(robot_face_raylib PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()

# ============================================================================
# C Version - Modular
# ============================================================================
if(BUILD_C_MODULAR)
    message(STATUS "Building modular C version")

    add_executable(robot_face_c
        src/main.c
        src/robot_face.c
        src/robot_face_draw.c
    )

    target_include_directories(robot_face_c PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include
    )

    target_link_libraries(robot_face_c
        ${RAYLIB_LIBRARIES}
        m  # Math library
    )

    # Platform-specific libraries
    if(APPLE)
        target_link_libraries(robot_face_c
            "-framework IOKit"
            "-framework Cocoa"
            "-framework OpenGL"
        )
    elseif(UNIX)
        target_link_libraries(robot_face_c
            GL
            pthread
            dl
            rt
            X11
        )
    endif()

    # Compiler flags
    target_compile_options(robot_face_c PRIVATE
        -Wall
        -Wextra
        -Wpedantic
    )

    # Copy to root build directory
    set_target_properties(robot_face_c PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()

# ============================================================================
# C++ Version - Modern
# ============================================================================
if(BUILD_CPP_MODERN)
    message(STATUS "Building modern C++ version")

    add_executable(robot_face_cpp
        src/main.cpp
        src/robot_face.cpp
    )

    target_include_directories(robot_face_cpp PRIVATE
        ${CMAKE_CURRENT_SOURCE_DIR}/include
    )

    target_link_libraries(robot_face_cpp
        ${RAYLIB_LIBRARIES}
        m  # Math library
    )

    # Platform-specific libraries
    if(APPLE)
        target_link_libraries(robot_face_cpp
            "-framework IOKit"
            "-framework Cocoa"
            "-framework OpenGL"
        )
    elseif(UNIX)
        target_link_libraries(robot_face_cpp
            GL
            pthread
            dl
            rt
            X11
        )
    endif()

    # Compiler flags
    target_compile_options(robot_face_cpp PRIVATE
        -Wall
        -Wextra
        -Wpedantic
    )

    # Copy to root build directory
    set_target_properties(robot_face_cpp PROPERTIES
        RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
    )
endif()

# ============================================================================
# Installation
# ============================================================================
if(BUILD_ORIGINAL)
    install(TARGETS robot_face_raylib DESTINATION bin)
endif()

if(BUILD_C_MODULAR)
    install(TARGETS robot_face_c DESTINATION bin)
endif()

if(BUILD_CPP_MODERN)
    install(TARGETS robot_face_cpp DESTINATION bin)
endif()

install(FILES
    include/robot_face.h
    include/robot_face.hpp
    include/robot_face_config.h
    DESTINATION include
)

# ============================================================================
# Print configuration
# ============================================================================
message(STATUS "")
message(STATUS "Robot Face Raylib Configuration:")
message(STATUS "  C Standard: ${CMAKE_C_STANDARD}")
message(STATUS "  C++ Standard: ${CMAKE_CXX_STANDARD}")
message(STATUS "  Build Original: ${BUILD_ORIGINAL}")
message(STATUS "  Build C Modular: ${BUILD_C_MODULAR}")
message(STATUS "  Build C++ Modern: ${BUILD_CPP_MODERN}")
message(STATUS "  Raylib Include: ${RAYLIB_INCLUDE_DIRS}")
message(STATUS "  Raylib Library: ${RAYLIB_LIBRARIES}")
message(STATUS "")
