/*******************************************************************************************
 *
 *   Robot Face - Modern C++ Implementation
 *
 *******************************************************************************************/

#include "robot_face.hpp"
#include <algorithm>
#include <cmath>

namespace robotface {

// Constructor
RobotFace::RobotFace(float initialHappiness)
    : m_happiness(clampHappiness(initialHappiness))
    , m_blinkProgress(0.0f)
    , m_blinkTimer(0.0)
    , m_isBlinking(false)
{
    // All initialization done in member initializer list (modern C++ style)
}

// Update animation state
void RobotFace::update(float deltaTime) {
    // Update blink timer for automatic blinking
    m_blinkTimer += deltaTime;

    if (m_blinkTimer >= Config::BLINK_INTERVAL && !m_isBlinking) {
        m_isBlinking = true;
        m_blinkTimer = 0.0;
    }

    // Animate blink
    if (m_isBlinking) {
        m_blinkProgress += Config::BLINK_SPEED * deltaTime;

        if (m_blinkProgress >= Config::BLINK_COMPLETE_THRESHOLD) {
            m_blinkProgress = 0.0f;
            m_isBlinking = false;
        }
    }
}

// Set emotion by float value
void RobotFace::setEmotion(float happiness) {
    m_happiness = clampHappiness(happiness);
}

// Set emotion by enum
void RobotFace::setEmotion(Emotion emotion) {
    m_happiness = emotionToHappiness(emotion);
}

// Trigger manual blink
void RobotFace::triggerBlink() {
    if (!m_isBlinking) {
        m_isBlinking = true;
        m_blinkProgress = 0.0f;
    }
}

// Get current emotion based on happiness level
Emotion RobotFace::currentEmotion() const noexcept {
    if (m_happiness > Config::EMOTION_HAPPY_THRESHOLD) {
        return Emotion::Happy;
    } else if (m_happiness < Config::EMOTION_SAD_THRESHOLD) {
        return Emotion::Sad;
    } else {
        return Emotion::Neutral;
    }
}

// Get emotion name as string
std::string RobotFace::emotionName() const {
    switch (currentEmotion()) {
        case Emotion::Happy: return "Happy";
        case Emotion::Sad: return "Sad";
        case Emotion::Neutral: return "Neutral";
        default: return "Unknown";
    }
}

// Handle keyboard input
void RobotFace::handleKeyboardInput() {
    if (IsKeyPressed(KEY_H)) setEmotion(Emotion::Happy);
    if (IsKeyPressed(KEY_N)) setEmotion(Emotion::Neutral);
    if (IsKeyPressed(KEY_S)) setEmotion(Emotion::Sad);
}

// Handle mouse input
void RobotFace::handleMouseInput() {
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
        triggerBlink();
    }
}

// Check if mouse is over mouth area
bool RobotFace::isMouseOverMouth() const {
    Vector2 mousePos = GetMousePosition();
    return CheckCollisionPointRec(mousePos, Config::HOVER_AREA);
}

// Calculate blink factor for animation
float RobotFace::calculateBlinkFactor(float progress) const noexcept {
    if (progress < 1.0f) {
        // Closing (0 to 1)
        return std::sin(progress * PI / 2.0f);
    } else {
        // Opening (1 to 0)
        return std::sin((2.0f - progress) * PI / 2.0f);
    }
}

// Clamp happiness to valid range [0, 1]
float RobotFace::clampHappiness(float value) noexcept {
    return std::clamp(value, 0.0f, 1.0f);
}

// Draw a single eye with blink animation
void RobotFace::drawEye(float x, float y, float blinkProgress) const {
    const float blinkFactor = calculateBlinkFactor(blinkProgress);

    // Eye white (outer circle)
    DrawCircle(static_cast<int>(x), static_cast<int>(y), Config::EYE_RADIUS, WHITE);
    DrawCircleLines(static_cast<int>(x), static_cast<int>(y), Config::EYE_RADIUS, BLACK);

    // Pupil size changes during blink
    const float pupilRadius = Config::PUPIL_RADIUS * (1.0f - blinkFactor * 0.875f);

    // Pupil (black circle)
    DrawCircle(static_cast<int>(x), static_cast<int>(y), pupilRadius, BLACK);

    // Highlight (gives eyes a "shiny" look)
    if (pupilRadius > 10.0f) {
        const float highlightSize = Config::HIGHLIGHT_RADIUS * (pupilRadius / Config::PUPIL_RADIUS);
        DrawCircle(static_cast<int>(x - 15), static_cast<int>(y - 15), highlightSize, WHITE);
    }
}

// Draw mouth as a Bezier curve
void RobotFace::drawMouth(float centerX, float centerY, float happiness) const {
    const Vector2 start = Config::MOUTH_START;
    const Vector2 end = Config::MOUTH_END;

    // Control point Y varies with emotion
    const float controlY = Config::MOUTH_CENTER.y + (happiness - 0.5f) * Config::MOUTH_CURVE_FACTOR;
    const Vector2 control = { Config::MOUTH_CENTER.x, controlY };

    // Draw Bezier curve with multiple segments for smoothness
    for (int i = 0; i < Config::MOUTH_SEGMENTS; i++) {
        const float t1 = static_cast<float>(i) / Config::MOUTH_SEGMENTS;
        const float t2 = static_cast<float>(i + 1) / Config::MOUTH_SEGMENTS;

        // Quadratic Bezier formula: B(t) = (1-t)²P0 + 2(1-t)tP1 + t²P2
        const float x1 = (1-t1)*(1-t1)*start.x + 2*(1-t1)*t1*control.x + t1*t1*end.x;
        const float y1 = (1-t1)*(1-t1)*start.y + 2*(1-t1)*t1*control.y + t1*t1*end.y;

        const float x2 = (1-t2)*(1-t2)*start.x + 2*(1-t2)*t2*control.x + t2*t2*end.x;
        const float y2 = (1-t2)*(1-t2)*start.y + 2*(1-t2)*t2*control.y + t2*t2*end.y;

        DrawLineEx(Vector2{x1, y1}, Vector2{x2, y2}, Config::MOUTH_STROKE_WIDTH, BLACK);
    }
}

// Draw UI elements (title, emotion, FPS, controls)
void RobotFace::drawUI(int width, int height) const {
    DrawText("Raylib Robot Face (Modern C++)", 10, 10, 20, DARKGRAY);

    const std::string emotionText = "Emotion: " + emotionName() +
                                   " (" + std::to_string(m_happiness).substr(0, 4) + ")";
    DrawText(emotionText.c_str(), 10, 40, 20, DARKGRAY);

    DrawText(TextFormat("FPS: %d", GetFPS()), 10, 70, 20, DARKGREEN);
    DrawText("Controls: H=Happy, S=Sad, N=Neutral, Click=Blink, ESC=Exit",
             10, height - 30, 16, GRAY);
}

// Draw complete robot face
void RobotFace::draw(int width, int height) const {
    ClearBackground(RAYWHITE);

    // Draw eyes
    drawEye(Config::LEFT_EYE_POS.x, Config::LEFT_EYE_POS.y, m_blinkProgress);
    drawEye(Config::RIGHT_EYE_POS.x, Config::RIGHT_EYE_POS.y, m_blinkProgress);

    // Draw mouth
    drawMouth(Config::MOUTH_CENTER.x, Config::MOUTH_CENTER.y, m_happiness);

    // Draw UI
    drawUI(width, height);
}

} // namespace robotface
