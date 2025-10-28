/*******************************************************************************************
 *
 *   Robot Face - Main Entry Point (Modular C Version)
 *
 *   Controls:
 *   - H: Happy emotion
 *   - S: Sad emotion
 *   - N: Neutral emotion
 *   - Mouse Click: Trigger blink
 *   - ESC: Exit
 *
 *******************************************************************************************/

#include "robot_face.h"
#include "robot_face_config.h"
#include "raylib.h"
#include <math.h>

int main(void) {
    // Initialization
    InitWindow(SCREEN_WIDTH, SCREEN_HEIGHT, "Robot Face - Raylib (Modular C)");
    SetTargetFPS(60);

    RobotFace face;
    InitRobotFace(&face);

    // Main game loop
    while (!WindowShouldClose()) {
        // Update
        float deltaTime = GetFrameTime();
        UpdateRobotFace(&face, deltaTime);

        // Keyboard controls
        if (IsKeyPressed(KEY_H)) SetEmotion(&face, 1.0f);  // Happy
        if (IsKeyPressed(KEY_N)) SetEmotion(&face, 0.5f);  // Neutral
        if (IsKeyPressed(KEY_S)) SetEmotion(&face, 0.0f);  // Sad

        // Mouse controls
        if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
            TriggerBlink(&face);
        }

        // Mouse hover effect (wider smile when hovering over mouth area)
        Vector2 mousePos = GetMousePosition();
        if (mousePos.x > HOVER_AREA_MIN_X && mousePos.x < HOVER_AREA_MAX_X &&
            mousePos.y > HOVER_AREA_MIN_Y && mousePos.y < HOVER_AREA_MAX_Y) {
            // Gradually increase happiness when hovering
            float newHappiness = face.happiness + deltaTime * HOVER_HAPPINESS_SPEED;
            if (newHappiness > 1.0f) newHappiness = 1.0f;
            face.happiness = newHappiness;
        }

        // Draw
        BeginDrawing();
        DrawRobotFace(&face, SCREEN_WIDTH, SCREEN_HEIGHT);
        EndDrawing();
    }

    // De-Initialization
    CloseWindow();

    return 0;
}
