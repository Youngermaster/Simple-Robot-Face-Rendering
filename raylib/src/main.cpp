/*******************************************************************************************
 *
 *   Robot Face - Main Entry Point (Modern C++ Version)
 *
 *   Controls:
 *   - H: Happy emotion
 *   - S: Sad emotion
 *   - N: Neutral emotion
 *   - Mouse Click: Trigger blink
 *   - ESC: Exit
 *
 *******************************************************************************************/

#include "robot_face.hpp"
#include <algorithm>

int main() {
    using namespace robotface;

    // Initialization
    InitWindow(Config::SCREEN_WIDTH, Config::SCREEN_HEIGHT, "Robot Face - Raylib (Modern C++)");
    SetTargetFPS(60);

    // Create robot face with RAII (automatic cleanup on scope exit)
    RobotFace face(0.8f);  // Start with happiness = 0.8

    // Main game loop
    while (!WindowShouldClose()) {
        // Get delta time
        const float deltaTime = GetFrameTime();

        // Update face animation
        face.update(deltaTime);

        // Handle keyboard input
        face.handleKeyboardInput();

        // Handle mouse input
        face.handleMouseInput();

        // Mouse hover effect (wider smile when hovering over mouth area)
        if (face.isMouseOverMouth()) {
            // Gradually increase happiness when hovering
            const float newHappiness = std::min(face.happiness() + deltaTime * Config::HOVER_HAPPINESS_SPEED, 1.0f);
            face.setEmotion(newHappiness);
        }

        // Draw
        BeginDrawing();
        face.draw(Config::SCREEN_WIDTH, Config::SCREEN_HEIGHT);
        EndDrawing();
    }

    // De-Initialization (automatic via RAII)
    CloseWindow();

    return 0;
}
