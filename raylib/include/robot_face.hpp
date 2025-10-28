/*******************************************************************************************
 *
 *   Robot Face - Modern C++ Interface
 *
 *   Features:
 *   - RAII design (automatic resource management)
 *   - Const correctness
 *   - Strong typing with enums
 *   - Compile-time constants
 *
 *******************************************************************************************/

#ifndef ROBOT_FACE_HPP
#define ROBOT_FACE_HPP

#include "raylib.h"
#include <string>

namespace robotface {

// Emotion presets as enum class (C++11 strong typing)
enum class Emotion {
    Sad = 0,
    Neutral,
    Happy
};

// Robot face configuration (compile-time constants)
struct Config {
    // Screen
    static constexpr int SCREEN_WIDTH = 800;
    static constexpr int SCREEN_HEIGHT = 600;

    // Eye parameters
    static constexpr float EYE_RADIUS = 60.0f;
    static constexpr float PUPIL_RADIUS = 40.0f;
    static constexpr float PUPIL_MIN_RADIUS = 5.0f;
    static constexpr float HIGHLIGHT_RADIUS = 15.0f;
    static constexpr Vector2 LEFT_EYE_POS = {250.0f, 200.0f};
    static constexpr Vector2 RIGHT_EYE_POS = {550.0f, 200.0f};

    // Mouth parameters
    static constexpr Vector2 MOUTH_START = {300.0f, 400.0f};
    static constexpr Vector2 MOUTH_END = {500.0f, 400.0f};
    static constexpr Vector2 MOUTH_CENTER = {400.0f, 400.0f};
    static constexpr float MOUTH_STROKE_WIDTH = 8.0f;
    static constexpr float MOUTH_CURVE_FACTOR = 60.0f;
    static constexpr int MOUTH_SEGMENTS = 30;

    // Animation
    static constexpr float BLINK_INTERVAL = 3.0f;
    static constexpr float BLINK_SPEED = 5.0f;
    static constexpr float BLINK_COMPLETE_THRESHOLD = 2.0f;
    static constexpr float HOVER_HAPPINESS_SPEED = 0.5f;

    // Hover area (mouth region)
    static constexpr Rectangle HOVER_AREA = {300.0f, 350.0f, 200.0f, 100.0f};

    // Emotion thresholds
    static constexpr float EMOTION_HAPPY_THRESHOLD = 0.7f;
    static constexpr float EMOTION_SAD_THRESHOLD = 0.3f;
};

// Robot face class with RAII design
class RobotFace {
public:
    // Constructor with default happiness value
    explicit RobotFace(float initialHappiness = 0.8f);

    // Destructor (default is fine, no manual cleanup needed)
    ~RobotFace() = default;

    // Delete copy constructor and assignment (unique face instance)
    RobotFace(const RobotFace&) = delete;
    RobotFace& operator=(const RobotFace&) = delete;

    // Allow move semantics (C++11)
    RobotFace(RobotFace&&) = default;
    RobotFace& operator=(RobotFace&&) = default;

    // Core update and rendering
    void update(float deltaTime);
    void draw(int width, int height) const;

    // Emotion control
    void setEmotion(float happiness);
    void setEmotion(Emotion emotion);
    void triggerBlink();

    // State queries (const methods)
    [[nodiscard]] float happiness() const noexcept { return m_happiness; }
    [[nodiscard]] bool isBlinking() const noexcept { return m_isBlinking; }
    [[nodiscard]] float blinkProgress() const noexcept { return m_blinkProgress; }
    [[nodiscard]] Emotion currentEmotion() const noexcept;
    [[nodiscard]] std::string emotionName() const;

    // Interaction helpers
    void handleKeyboardInput();
    void handleMouseInput();
    bool isMouseOverMouth() const;

private:
    // State variables (initialized in constructor)
    float m_happiness = 0.8f;
    float m_blinkProgress = 0.0f;
    double m_blinkTimer = 0.0;
    bool m_isBlinking = false;

    // Private drawing methods (const because they don't modify state)
    void drawEye(float x, float y, float blinkProgress) const;
    void drawMouth(float centerX, float centerY, float happiness) const;
    void drawUI(int width, int height) const;

    // Helper to calculate blink factor
    [[nodiscard]] float calculateBlinkFactor(float progress) const noexcept;

    // Helper to clamp happiness value
    static float clampHappiness(float value) noexcept;
};

// Utility function to convert emotion enum to float
inline float emotionToHappiness(Emotion emotion) noexcept {
    switch (emotion) {
        case Emotion::Sad: return 0.0f;
        case Emotion::Neutral: return 0.5f;
        case Emotion::Happy: return 1.0f;
        default: return 0.5f;
    }
}

} // namespace robotface

#endif // ROBOT_FACE_HPP
