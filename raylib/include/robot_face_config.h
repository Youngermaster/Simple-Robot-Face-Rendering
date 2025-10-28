/*******************************************************************************************
 *
 *   Robot Face Configuration - Constants and Parameters
 *
 *******************************************************************************************/

#ifndef ROBOT_FACE_CONFIG_H
#define ROBOT_FACE_CONFIG_H

#ifdef __cplusplus
extern "C" {
#endif

// Screen dimensions
#define SCREEN_WIDTH 800
#define SCREEN_HEIGHT 600

// Eye parameters
#define EYE_RADIUS 60.0f
#define PUPIL_RADIUS 40.0f
#define PUPIL_MIN_RADIUS 5.0f
#define HIGHLIGHT_RADIUS 15.0f
#define HIGHLIGHT_OFFSET_X -15.0f
#define HIGHLIGHT_OFFSET_Y -15.0f

// Eye positions
#define LEFT_EYE_X 250.0f
#define LEFT_EYE_Y 200.0f
#define RIGHT_EYE_X 550.0f
#define RIGHT_EYE_Y 200.0f

// Mouth parameters
#define MOUTH_START_X 300.0f
#define MOUTH_START_Y 400.0f
#define MOUTH_END_X 500.0f
#define MOUTH_END_Y 400.0f
#define MOUTH_CENTER_X 400.0f
#define MOUTH_CENTER_Y 400.0f
#define MOUTH_STROKE_WIDTH 8.0f
#define MOUTH_CURVE_FACTOR 60.0f
#define MOUTH_SEGMENTS 30

// Animation parameters
#define BLINK_INTERVAL 3.0f
#define BLINK_SPEED 5.0f
#define BLINK_COMPLETE_THRESHOLD 2.0f
#define HOVER_HAPPINESS_SPEED 0.5f

// Hover detection area (mouth region)
#define HOVER_AREA_MIN_X 300.0f
#define HOVER_AREA_MAX_X 500.0f
#define HOVER_AREA_MIN_Y 350.0f
#define HOVER_AREA_MAX_Y 450.0f

// Emotion thresholds
#define EMOTION_HAPPY_THRESHOLD 0.7f
#define EMOTION_SAD_THRESHOLD 0.3f

#ifdef __cplusplus
}
#endif

#endif // ROBOT_FACE_CONFIG_H
