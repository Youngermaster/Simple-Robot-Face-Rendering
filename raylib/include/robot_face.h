/*******************************************************************************************
 *
 *   Robot Face - Public Interface (C API)
 *
 *   Features:
 *   - Animated robot face with eyes and mouth
 *   - Automatic blinking every 3 seconds
 *   - Interactive emotion control
 *
 *******************************************************************************************/

#ifndef ROBOT_FACE_H
#define ROBOT_FACE_H

#include <stdbool.h>

#ifdef __cplusplus
extern "C" {
#endif

// Robot face state structure
typedef struct {
    float happiness;         // 0.0 (sad) to 1.0 (happy)
    float blink_progress;    // 0.0 (open) to 1.0 (closed)
    double blink_timer;      // Time accumulator for automatic blinking
    bool is_blinking;        // Whether currently in a blink animation
    float blink_speed;       // Speed of blink animation
} RobotFace;

// Core functions
void InitRobotFace(RobotFace* face);
void UpdateRobotFace(RobotFace* face, float deltaTime);
void DrawRobotFace(RobotFace* face, int width, int height);

// Emotion control
void SetEmotion(RobotFace* face, float happiness);
void TriggerBlink(RobotFace* face);

// Getters
const char* GetEmotionName(const RobotFace* face);
float GetHappiness(const RobotFace* face);
bool IsBlinking(const RobotFace* face);

#ifdef __cplusplus
}
#endif

#endif // ROBOT_FACE_H
