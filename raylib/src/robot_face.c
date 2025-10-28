/*******************************************************************************************
 *
 *   Robot Face - Core Logic Implementation
 *
 *******************************************************************************************/

#include "robot_face.h"
#include "robot_face_config.h"
#include <math.h>

// Initialize robot face with default values
void InitRobotFace(RobotFace* face) {
    face->happiness = 0.8f;        // Start slightly happy
    face->blink_progress = 0.0f;   // Eyes open
    face->blink_timer = 0.0;       // Reset timer
    face->is_blinking = false;     // Not blinking
    face->blink_speed = BLINK_SPEED;
}

// Update robot face state (animations)
void UpdateRobotFace(RobotFace* face, float deltaTime) {
    // Update blink timer for automatic blinking
    face->blink_timer += deltaTime;

    if (face->blink_timer >= BLINK_INTERVAL && !face->is_blinking) {
        face->is_blinking = true;
        face->blink_timer = 0.0;
    }

    // Animate blink
    if (face->is_blinking) {
        // Blink animation: 0 -> 1 -> 0 (smooth sine wave)
        face->blink_progress += face->blink_speed * deltaTime;

        if (face->blink_progress >= BLINK_COMPLETE_THRESHOLD) {
            face->blink_progress = 0.0f;
            face->is_blinking = false;
        }
    }
}

// Set emotion manually
void SetEmotion(RobotFace* face, float happiness) {
    // Clamp to [0, 1]
    if (happiness < 0.0f) happiness = 0.0f;
    if (happiness > 1.0f) happiness = 1.0f;
    face->happiness = happiness;
}

// Trigger manual blink
void TriggerBlink(RobotFace* face) {
    if (!face->is_blinking) {
        face->is_blinking = true;
        face->blink_progress = 0.0f;
    }
}

// Get emotion name based on happiness level
const char* GetEmotionName(const RobotFace* face) {
    if (face->happiness > EMOTION_HAPPY_THRESHOLD) {
        return "Happy";
    } else if (face->happiness < EMOTION_SAD_THRESHOLD) {
        return "Sad";
    } else {
        return "Neutral";
    }
}

// Get current happiness value
float GetHappiness(const RobotFace* face) {
    return face->happiness;
}

// Check if currently blinking
bool IsBlinking(const RobotFace* face) {
    return face->is_blinking;
}
