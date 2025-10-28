/*******************************************************************************************
 *
 *   Robot Face - Drawing Functions Implementation
 *
 *******************************************************************************************/

#include "robot_face.h"
#include "robot_face_config.h"
#include "raylib.h"
#include <math.h>
#include <stdio.h>

// Draw a single eye with blink animation
static void DrawEye(float x, float y, float blinkProgress) {
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
    DrawCircle((int)x, (int)y, EYE_RADIUS, WHITE);
    DrawCircleLines((int)x, (int)y, EYE_RADIUS, BLACK);

    // Pupil size changes during blink
    float pupilRadius = PUPIL_RADIUS * (1.0f - blinkFactor * 0.875f); // Shrinks to 5px when closed

    // Pupil (black circle)
    DrawCircle((int)x, (int)y, pupilRadius, BLACK);

    // Highlight (gives eyes a "shiny" look)
    if (pupilRadius > 10.0f) {
        float highlightSize = HIGHLIGHT_RADIUS * (pupilRadius / PUPIL_RADIUS);
        DrawCircle((int)(x + HIGHLIGHT_OFFSET_X), (int)(y + HIGHLIGHT_OFFSET_Y), highlightSize, WHITE);
    }
}

// Draw mouth as a Bezier curve
static void DrawMouth(float centerX, float centerY, float happiness) {
    // Mouth positions
    Vector2 start = { MOUTH_START_X, MOUTH_START_Y };
    Vector2 end = { MOUTH_END_X, MOUTH_END_Y };

    // Control point Y varies with emotion
    // happiness 1.0 (happy) -> Y = 430 (curve down)
    // happiness 0.5 (neutral) -> Y = 400 (straight)
    // happiness 0.0 (sad) -> Y = 370 (curve up)
    float controlY = MOUTH_CENTER_Y + (happiness - 0.5f) * MOUTH_CURVE_FACTOR;
    Vector2 control = { MOUTH_CENTER_X, controlY };

    // Draw Bezier curve with multiple segments for smoothness
    for (int i = 0; i < MOUTH_SEGMENTS; i++) {
        float t1 = (float)i / MOUTH_SEGMENTS;
        float t2 = (float)(i + 1) / MOUTH_SEGMENTS;

        // Quadratic Bezier formula: B(t) = (1-t)²P0 + 2(1-t)tP1 + t²P2
        float x1 = (1-t1)*(1-t1)*start.x + 2*(1-t1)*t1*control.x + t1*t1*end.x;
        float y1 = (1-t1)*(1-t1)*start.y + 2*(1-t1)*t1*control.y + t1*t1*end.y;

        float x2 = (1-t2)*(1-t2)*start.x + 2*(1-t2)*t2*control.x + t2*t2*end.x;
        float y2 = (1-t2)*(1-t2)*start.y + 2*(1-t2)*t2*control.y + t2*t2*end.y;

        DrawLineEx((Vector2){x1, y1}, (Vector2){x2, y2}, MOUTH_STROKE_WIDTH, BLACK);
    }
}

// Draw complete robot face
void DrawRobotFace(RobotFace* face, int width, int height) {
    // Clear background
    ClearBackground(RAYWHITE);

    // Draw title
    DrawText("Raylib Robot Face (Modular C)", 10, 10, 20, DARKGRAY);

    // Draw eyes
    DrawEye(LEFT_EYE_X, LEFT_EYE_Y, face->blink_progress);
    DrawEye(RIGHT_EYE_X, RIGHT_EYE_Y, face->blink_progress);

    // Draw mouth
    DrawMouth(MOUTH_CENTER_X, MOUTH_CENTER_Y, face->happiness);

    // Draw emotion indicator
    const char* emotion = GetEmotionName(face);
    DrawText(TextFormat("Emotion: %s (%.2f)", emotion, face->happiness), 10, 40, 20, DARKGRAY);
    DrawText(TextFormat("FPS: %d", GetFPS()), 10, 70, 20, DARKGREEN);

    // Draw controls
    DrawText("Controls: H=Happy, S=Sad, N=Neutral, Click=Blink, ESC=Exit", 10, height - 30, 16, GRAY);
}
