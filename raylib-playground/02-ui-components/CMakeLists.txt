cmake_minimum_required(VERSION 3.15)
project(UIComponents VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Find Raylib
if(APPLE)
    list(APPEND CMAKE_PREFIX_PATH "/opt/homebrew" "/usr/local")
endif()

find_package(raylib QUIET)

if (raylib_FOUND)
    message(STATUS "Found raylib via CMake config")
    set(RAYLIB_LIBRARIES raylib)
else()
    find_package(PkgConfig)
    if (PkgConfig_FOUND)
        pkg_check_modules(RAYLIB QUIET raylib)
        if (RAYLIB_FOUND)
            message(STATUS "Found raylib via pkg-config")
        endif()
    endif()

    if (NOT RAYLIB_FOUND)
        message(STATUS "Raylib not found via CMake or pkg-config, using manual paths...")
        set(RAYLIB_INCLUDE_DIRS "/opt/homebrew/include")
        set(RAYLIB_LIBRARY_DIRS "/opt/homebrew/lib")
        set(RAYLIB_LIBRARIES "raylib")

        if(EXISTS "${RAYLIB_INCLUDE_DIRS}/raylib.h" AND EXISTS "${RAYLIB_LIBRARY_DIRS}/libraylib.dylib")
            message(STATUS "Verified raylib installation at ${RAYLIB_INCLUDE_DIRS}")
        else()
            message(FATAL_ERROR "Raylib not found! Please install with: brew install raylib")
        endif()
    endif()
endif()

# Source files
set(SOURCES
    src/main.cpp
    src/button.cpp
    src/color_picker.cpp
    src/theme.cpp
)

# Create executable
add_executable(ui_components ${SOURCES})

# Include directories
target_include_directories(ui_components PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/include
    ${RAYLIB_INCLUDE_DIRS}
)

# Link directories
if(DEFINED RAYLIB_LIBRARY_DIRS)
    link_directories(${RAYLIB_LIBRARY_DIRS})
endif()

# Link libraries
target_link_libraries(ui_components
    ${RAYLIB_LIBRARIES}
    m
)

# Platform-specific settings
if(APPLE)
    target_link_libraries(ui_components
        "-framework IOKit"
        "-framework Cocoa"
        "-framework OpenGL"
    )
elseif(UNIX)
    target_link_libraries(ui_components
        GL
        pthread
        dl
        rt
        X11
    )
endif()

# Compiler flags
target_compile_options(ui_components PRIVATE
    -Wall
    -Wextra
    -Wpedantic
)
