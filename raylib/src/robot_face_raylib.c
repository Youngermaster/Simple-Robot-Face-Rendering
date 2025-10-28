/*******************************************************************************************
 *
 *   Robot Face Rendering - Raylib Implementation
 *
 *   Features:
 *   - Animated robot face with eyes and mouth
 *   - Automatic blinking every 3 seconds
 *   - Interactive emotion control (keyboard H/S/N or mouse hover)
 *   - Click to trigger manual blink
 *
 *   Controls:
 *   - H: Happy emotion
 *   - S: Sad emotion
 *   - N: Neutral emotion
 *   - Mouse Click: Trigger blink
 *   - ESC: Exit
 *
 *******************************************************************************************/

#include "raylib.h"
#include <math.h>
#include <stdio.h>

// Robot face state structure
typedef struct {
    float happiness;         // 0.0 (sad) to 1.0 (happy)
    float blink_progress;    // 0.0 (open) to 1.0 (closed)
    double blink_timer;      // Time accumulator for automatic blinking
    bool is_blinking;        // Whether currently in a blink animation
    float blink_speed;       // Speed of blink animation
} RobotFace;

// Initialize robot face with default values
void InitRobotFace(RobotFace* face) {
    face->happiness = 0.8f;        // Start slightly happy
    face->blink_progress = 0.0f;   // Eyes open
    face->blink_timer = 0.0;       // Reset timer
    face->is_blinking = false;     // Not blinking
    face->blink_speed = 5.0f;      // Blink completes in 0.2 seconds (5 * 0.04 = 0.2)
}

// Update robot face state (animations)
void UpdateRobotFace(RobotFace* face, float deltaTime) {
    // Update blink timer for automatic blinking (every 3 seconds)
    face->blink_timer += deltaTime;

    if (face->blink_timer >= 3.0 && !face->is_blinking) {
        face->is_blinking = true;
        face->blink_timer = 0.0;
    }

    // Animate blink
    if (face->is_blinking) {
        // Blink animation: 0 -> 1 -> 0 (smooth sine wave)
        face->blink_progress += face->blink_speed * deltaTime;

        if (face->blink_progress >= 2.0f) {
            face->blink_progress = 0.0f;
            face->is_blinking = false;
        }
    }
}

// Draw a single eye with blink animation
void DrawEye(float x, float y, float blinkProgress, int width, int height) {
    // Calculate blink factor (0 = open, 1 = closed)
    // Use sine wave for smooth animation
    float blinkFactor = 0.0f;
    if (blinkProgress < 1.0f) {
        // Closing (0 to 1)
        blinkFactor = sinf(blinkProgress * PI / 2.0f);
    } else {
        // Opening (1 to 0)
        blinkFactor = sinf((2.0f - blinkProgress) * PI / 2.0f);
    }

    // Eye white (outer circle)
    DrawCircle((int)x, (int)y, 60, WHITE);
    DrawCircleLines((int)x, (int)y, 60, BLACK);

    // Pupil size changes during blink
    float pupilRadius = 40.0f * (1.0f - blinkFactor * 0.875f); // Shrinks to 5px when closed

    // Pupil (black circle)
    DrawCircle((int)x, (int)y, pupilRadius, BLACK);

    // Highlight (gives eyes a "shiny" look)
    if (pupilRadius > 10.0f) {
        float highlightSize = 15.0f * (pupilRadius / 40.0f);
        DrawCircle((int)(x - 15), (int)(y - 15), highlightSize, WHITE);
    }
}

// Draw mouth as a Bezier curve
void DrawMouth(float centerX, float centerY, float happiness, int width, int height) {
    // Mouth positions
    Vector2 start = { 300, 400 };
    Vector2 end = { 500, 400 };

    // Control point Y varies with emotion
    // happiness 1.0 (happy) -> Y = 430 (curve down)
    // happiness 0.5 (neutral) -> Y = 400 (straight)
    // happiness 0.0 (sad) -> Y = 370 (curve up)
    float controlY = 400.0f + (happiness - 0.5f) * 60.0f;
    Vector2 control = { 400, controlY };

    // Draw Bezier curve with multiple segments for smoothness
    int segments = 30;
    for (int i = 0; i < segments; i++) {
        float t1 = (float)i / segments;
        float t2 = (float)(i + 1) / segments;

        // Quadratic Bezier formula: B(t) = (1-t)²P0 + 2(1-t)tP1 + t²P2
        float x1 = (1-t1)*(1-t1)*start.x + 2*(1-t1)*t1*control.x + t1*t1*end.x;
        float y1 = (1-t1)*(1-t1)*start.y + 2*(1-t1)*t1*control.y + t1*t1*end.y;

        float x2 = (1-t2)*(1-t2)*start.x + 2*(1-t2)*t2*control.x + t2*t2*end.x;
        float y2 = (1-t2)*(1-t2)*start.y + 2*(1-t2)*t2*control.y + t2*t2*end.y;

        DrawLineEx((Vector2){x1, y1}, (Vector2){x2, y2}, 8.0f, BLACK);
    }
}

// Draw complete robot face
void DrawRobotFace(RobotFace* face, int width, int height) {
    // Clear background
    ClearBackground(RAYWHITE);

    // Draw title
    DrawText("Raylib Robot Face", 10, 10, 20, DARKGRAY);

    // Draw eyes
    DrawEye(250, 200, face->blink_progress, width, height);
    DrawEye(550, 200, face->blink_progress, width, height);

    // Draw mouth
    DrawMouth(400, 400, face->happiness, width, height);

    // Draw emotion indicator
    const char* emotion = "Neutral";
    if (face->happiness > 0.7f) emotion = "Happy";
    else if (face->happiness < 0.3f) emotion = "Sad";

    DrawText(TextFormat("Emotion: %s (%.2f)", emotion, face->happiness), 10, 40, 20, DARKGRAY);
    DrawText(TextFormat("FPS: %d", GetFPS()), 10, 70, 20, DARKGREEN);

    // Draw controls
    DrawText("Controls: H=Happy, S=Sad, N=Neutral, Click=Blink, ESC=Exit", 10, height - 30, 16, GRAY);
}

// Set emotion manually
void SetEmotion(RobotFace* face, float happiness) {
    face->happiness = fmaxf(0.0f, fminf(1.0f, happiness)); // Clamp to [0, 1]
}

// Trigger manual blink
void TriggerBlink(RobotFace* face) {
    if (!face->is_blinking) {
        face->is_blinking = true;
        face->blink_progress = 0.0f;
    }
}

//------------------------------------------------------------------------------------
// Main entry point
//------------------------------------------------------------------------------------
int main(void) {
    // Initialization
    const int screenWidth = 800;
    const int screenHeight = 600;

    InitWindow(screenWidth, screenHeight, "Robot Face - Raylib");
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
        if (mousePos.x > 300 && mousePos.x < 500 && mousePos.y > 350 && mousePos.y < 450) {
            // Gradually increase happiness when hovering
            face.happiness = fminf(face.happiness + deltaTime * 0.5f, 1.0f);
        }

        // Draw
        BeginDrawing();
        DrawRobotFace(&face, screenWidth, screenHeight);
        EndDrawing();
    }

    // De-Initialization
    CloseWindow();

    return 0;
}
