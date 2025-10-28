/*******************************************************************************************
 *
 *   Robot Face Rendering - Skia Implementation
 *
 *   Features:
 *   - Animated robot face with eyes and mouth using Skia graphics
 *   - Automatic blinking every 3 seconds with smooth animation
 *   - Interactive emotion control (keyboard H/S/N or mouse hover)
 *   - High-quality antialiasing and advanced rendering effects
 *
 *   Controls:
 *   - H: Happy emotion
 *   - S: Sad emotion
 *   - N: Neutral emotion
 *   - Mouse Click: Trigger blink
 *   - ESC: Exit
 *
 *******************************************************************************************/

#include "include/core/SkCanvas.h"
#include "include/core/SkColor.h"
#include "include/core/SkPaint.h"
#include "include/core/SkPath.h"
#include "include/core/SkSurface.h"
#include "include/core/SkFont.h"
#include "include/core/SkFontMgr.h"
#include "include/effects/SkGradientShader.h"
#include "include/effects/SkImageFilters.h"
#include "tools/sk_app/Application.h"
#include "tools/sk_app/Window.h"

#include <chrono>
#include <cmath>
#include <sstream>
#include <iomanip>

using namespace sk_app;

class RobotFace {
public:
    RobotFace()
        : m_happiness(0.8f)
        , m_blinkProgress(0.0f)
        , m_blinkTimer(0.0)
        , m_isBlinking(false)
        , m_blinkSpeed(5.0f)
        , m_frameCount(0)
        , m_lastTime(std::chrono::steady_clock::now())
    {}

    void update(float deltaTime) {
        // Update frame counter for FPS calculation
        m_frameCount++;
        auto currentTime = std::chrono::steady_clock::now();
        float elapsed = std::chrono::duration<float>(currentTime - m_lastTime).count();
        if (elapsed >= 1.0f) {
            m_fps = m_frameCount / elapsed;
            m_frameCount = 0;
            m_lastTime = currentTime;
        }

        // Update blink timer for automatic blinking (every 3 seconds)
        m_blinkTimer += deltaTime;

        if (m_blinkTimer >= 3.0 && !m_isBlinking) {
            m_isBlinking = true;
            m_blinkTimer = 0.0;
        }

        // Animate blink
        if (m_isBlinking) {
            m_blinkProgress += m_blinkSpeed * deltaTime;

            if (m_blinkProgress >= 2.0f) {
                m_blinkProgress = 0.0f;
                m_isBlinking = false;
            }
        }
    }

    void draw(SkCanvas* canvas, int width, int height) {
        // Clear background
        canvas->clear(SK_ColorWHITE);

        // Draw title
        SkPaint textPaint;
        textPaint.setColor(SK_ColorDKGRAY);
        textPaint.setAntiAlias(true);

        SkFont font(nullptr, 20);
        canvas->drawString("Skia Robot Face", 10, 30, font, textPaint);

        // Draw eyes
        drawEye(canvas, 250, 200, m_blinkProgress);
        drawEye(canvas, 550, 200, m_blinkProgress);

        // Draw mouth
        drawMouth(canvas, 400, 400, m_happiness);

        // Draw emotion indicator
        const char* emotion = "Neutral";
        if (m_happiness > 0.7f) emotion = "Happy";
        else if (m_happiness < 0.3f) emotion = "Sad";

        std::ostringstream oss;
        oss << "Emotion: " << emotion << " (" << std::fixed << std::setprecision(2) << m_happiness << ")";
        canvas->drawString(oss.str().c_str(), 10, 60, font, textPaint);

        // Draw FPS
        std::ostringstream fpsText;
        fpsText << "FPS: " << static_cast<int>(m_fps);
        textPaint.setColor(SK_ColorGREEN);
        canvas->drawString(fpsText.str().c_str(), 10, 90, font, textPaint);

        // Draw controls
        font.setSize(16);
        textPaint.setColor(SK_ColorGRAY);
        canvas->drawString("Controls: H=Happy, S=Sad, N=Neutral, Click=Blink, ESC=Exit",
                         10, height - 20, font, textPaint);
    }

    void setEmotion(float happiness) {
        m_happiness = std::max(0.0f, std::min(1.0f, happiness));
    }

    void triggerBlink() {
        if (!m_isBlinking) {
            m_isBlinking = true;
            m_blinkProgress = 0.0f;
        }
    }

    float getHappiness() const { return m_happiness; }

private:
    void drawEye(SkCanvas* canvas, float x, float y, float blinkProgress) {
        // Calculate blink factor (0 = open, 1 = closed) using sine wave
        float blinkFactor = 0.0f;
        if (blinkProgress < 1.0f) {
            // Closing (0 to 1)
            blinkFactor = std::sin(blinkProgress * M_PI / 2.0f);
        } else {
            // Opening (1 to 0)
            blinkFactor = std::sin((2.0f - blinkProgress) * M_PI / 2.0f);
        }

        // Eye white (outer circle)
        SkPaint eyeWhitePaint;
        eyeWhitePaint.setColor(SK_ColorWHITE);
        eyeWhitePaint.setAntiAlias(true);
        eyeWhitePaint.setStyle(SkPaint::kFill_Style);
        canvas->drawCircle(x, y, 60, eyeWhitePaint);

        // Eye outline
        SkPaint outlinePaint;
        outlinePaint.setColor(SK_ColorBLACK);
        outlinePaint.setAntiAlias(true);
        outlinePaint.setStyle(SkPaint::kStroke_Style);
        outlinePaint.setStrokeWidth(2);
        canvas->drawCircle(x, y, 60, outlinePaint);

        // Pupil size changes during blink
        float pupilRadius = 40.0f * (1.0f - blinkFactor * 0.875f);

        // Pupil (black circle)
        SkPaint pupilPaint;
        pupilPaint.setColor(SK_ColorBLACK);
        pupilPaint.setAntiAlias(true);
        canvas->drawCircle(x, y, pupilRadius, pupilPaint);

        // Highlight (gives eyes a "shiny" look)
        if (pupilRadius > 10.0f) {
            float highlightSize = 15.0f * (pupilRadius / 40.0f);
            SkPaint highlightPaint;
            highlightPaint.setColor(SK_ColorWHITE);
            highlightPaint.setAntiAlias(true);

            // Add some transparency for a nice effect
            highlightPaint.setAlpha(200);
            canvas->drawCircle(x - 15, y - 15, highlightSize, highlightPaint);
        }
    }

    void drawMouth(SkCanvas* canvas, float centerX, float centerY, float happiness) {
        // Mouth positions
        SkPoint start = SkPoint::Make(300, 400);
        SkPoint end = SkPoint::Make(500, 400);

        // Control point Y varies with emotion
        float controlY = 400.0f + (happiness - 0.5f) * 60.0f;
        SkPoint control = SkPoint::Make(400, controlY);

        // Create quadratic bezier curve path
        SkPath mouthPath;
        mouthPath.moveTo(start);
        mouthPath.quadTo(control, end);

        // Draw mouth with high-quality stroke
        SkPaint mouthPaint;
        mouthPaint.setColor(SK_ColorBLACK);
        mouthPaint.setAntiAlias(true);
        mouthPaint.setStyle(SkPaint::kStroke_Style);
        mouthPaint.setStrokeWidth(8);
        mouthPaint.setStrokeCap(SkPaint::kRound_Cap);
        mouthPaint.setStrokeJoin(SkPaint::kRound_Join);

        canvas->drawPath(mouthPath, mouthPaint);
    }

    float m_happiness;
    float m_blinkProgress;
    double m_blinkTimer;
    bool m_isBlinking;
    float m_blinkSpeed;

    // FPS tracking
    int m_frameCount;
    std::chrono::steady_clock::time_point m_lastTime;
    float m_fps = 0.0f;
};

class RobotFaceApplication : public Application {
public:
    RobotFaceApplication(int argc, char** argv, void* platformData)
        : Application(argc, argv, platformData) {
        m_robotFace = std::make_unique<RobotFace>();
        m_lastFrameTime = std::chrono::steady_clock::now();
    }

    ~RobotFaceApplication() override = default;

    void onIdle() override {
        // Calculate delta time
        auto currentTime = std::chrono::steady_clock::now();
        float deltaTime = std::chrono::duration<float>(currentTime - m_lastFrameTime).count();
        m_lastFrameTime = currentTime;

        // Update robot face
        m_robotFace->update(deltaTime);

        // Request redraw
        if (fWindow) {
            fWindow->inval();
        }
    }

    void onPaint(SkSurface* surface) override {
        auto canvas = surface->getCanvas();
        m_robotFace->draw(canvas, fWindow->width(), fWindow->height());
    }

    void onChar(SkUnichar c, skui::ModifierKey modifiers) override {
        switch (c) {
            case 'h':
            case 'H':
                m_robotFace->setEmotion(1.0f); // Happy
                break;
            case 'n':
            case 'N':
                m_robotFace->setEmotion(0.5f); // Neutral
                break;
            case 's':
            case 'S':
                m_robotFace->setEmotion(0.0f); // Sad
                break;
        }
    }

    bool onMouse(int x, int y, skui::InputState state, skui::ModifierKey modifiers) override {
        if (state == skui::InputState::kDown) {
            // Mouse hover effect (wider smile when hovering over mouth area)
            if (x > 300 && x < 500 && y > 350 && y < 450) {
                float currentHappiness = m_robotFace->getHappiness();
                m_robotFace->setEmotion(std::min(currentHappiness + 0.1f, 1.0f));
            }

            // Trigger blink on click
            m_robotFace->triggerBlink();
        }
        return true;
    }

private:
    std::unique_ptr<RobotFace> m_robotFace;
    std::chrono::steady_clock::time_point m_lastFrameTime;
};

// Main entry point
Application* Application::Create(int argc, char** argv, void* platformData) {
    return new RobotFaceApplication(argc, argv, platformData);
}

int main(int argc, char** argv) {
    Application* app = Application::Create(argc, argv, nullptr);

    if (app) {
        // Create window
        app->fWindow = Window::CreateNativeWindow(nullptr);
        app->fWindow->attach(Window::kRaster_BackendType);
        app->fWindow->resize(800, 600);
        app->fWindow->setTitle("Robot Face - Skia");
        app->fWindow->show();

        // Run application
        while (!app->fWindow->shouldQuit()) {
            app->onIdle();
            app->fWindow->onPaint();
        }

        delete app;
    }

    return 0;
}
