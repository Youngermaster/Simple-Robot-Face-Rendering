cmake_minimum_required(VERSION 3.20)
project(RobotFaceRaylib C)

# Find raylib
find_package(raylib REQUIRED)

# Create executable
add_executable(robot_face_raylib
    src/robot_face_raylib.c
)

# Link raylib
target_link_libraries(robot_face_raylib
    raylib
)

# Include directories
target_include_directories(robot_face_raylib PRIVATE
    ${CMAKE_SOURCE_DIR}/common
)

# Platform-specific settings
if(APPLE)
    target_link_libraries(robot_face_raylib
        "-framework IOKit"
        "-framework Cocoa"
        "-framework OpenGL"
    )
endif()

# Copy to root build directory for easy access
set_target_properties(robot_face_raylib PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}
)
