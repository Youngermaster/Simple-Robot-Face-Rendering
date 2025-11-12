#include "curve_editor.hpp"

namespace Bezier {

CurveEditor::CurveEditor(int screenWidth, int screenHeight)
    : m_screenWidth(screenWidth)
    , m_screenHeight(screenHeight)
    , m_activeCurveIndex(0)
    , m_animationT(0.0f)
    , m_animating(false)
{
    // Start with one cubic curve
    addCurve(CurveType::Cubic);

    // Set default control points
    m_curves[0]->addControlPoint({200, 400});
    m_curves[0]->addControlPoint({300, 200});
    m_curves[0]->addControlPoint({700, 200});
    m_curves[0]->addControlPoint({800, 400});
}

void CurveEditor::update(float deltaTime) {
    handleInput();

    // Update active curve
    if (m_activeCurveIndex >= 0 && m_activeCurveIndex < static_cast<int>(m_curves.size())) {
        Vector2 mousePos = GetMousePosition();
        bool mousePressed = IsMouseButtonDown(MOUSE_LEFT_BUTTON);

        m_curves[m_activeCurveIndex]->update(mousePos, mousePressed);
    }

    // Update animation
    if (m_animating) {
        m_animationT += deltaTime * 0.5f;
        if (m_animationT > 1.0f) {
            m_animationT = 0.0f;
        }
    }
}

void CurveEditor::draw() const {
    ClearBackground(Color{20, 20, 30, 255});

    // Draw grid
    for (int x = 0; x < m_screenWidth; x += 50) {
        DrawLine(x, 0, x, m_screenHeight, ColorAlpha(GRAY, 0.1f));
    }
    for (int y = 0; y < m_screenHeight; y += 50) {
        DrawLine(0, y, m_screenWidth, y, ColorAlpha(GRAY, 0.1f));
    }

    // Draw all curves
    for (size_t i = 0; i < m_curves.size(); i++) {
        if (static_cast<int>(i) == m_activeCurveIndex) {
            m_curves[i]->draw();
        } else {
            // Draw inactive curves with transparency
            // (simplified drawing for inactive curves)
        }
    }

    // Draw animation point
    if (m_animating && !m_curves.empty()) {
        Vector2 animPoint = m_curves[m_activeCurveIndex]->getPointAt(m_animationT);
        DrawCircleV(animPoint, 12, RED);
        DrawCircleV(animPoint, 8, YELLOW);
    }

    drawUI();
}

void CurveEditor::addCurve(CurveType type) {
    m_curves.push_back(std::make_unique<BezierCurve>(type));
    m_activeCurveIndex = static_cast<int>(m_curves.size()) - 1;
}

void CurveEditor::clearCurves() {
    m_curves.clear();
    m_activeCurveIndex = -1;
}

void CurveEditor::loadPreset(int presetIndex) {
    if (m_curves.empty()) {
        addCurve(CurveType::Cubic);
    }

    m_curves[m_activeCurveIndex]->clearControlPoints();

    switch (presetIndex) {
        case 0: // Smile
            m_curves[m_activeCurveIndex]->addControlPoint({300, 400});
            m_curves[m_activeCurveIndex]->addControlPoint({400, 500});
            m_curves[m_activeCurveIndex]->addControlPoint({600, 500});
            m_curves[m_activeCurveIndex]->addControlPoint({700, 400});
            break;

        case 1: // Frown
            m_curves[m_activeCurveIndex]->addControlPoint({300, 400});
            m_curves[m_activeCurveIndex]->addControlPoint({400, 300});
            m_curves[m_activeCurveIndex]->addControlPoint({600, 300});
            m_curves[m_activeCurveIndex]->addControlPoint({700, 400});
            break;

        case 2: // S-Curve
            m_curves[m_activeCurveIndex]->addControlPoint({200, 200});
            m_curves[m_activeCurveIndex]->addControlPoint({400, 200});
            m_curves[m_activeCurveIndex]->addControlPoint({600, 600});
            m_curves[m_activeCurveIndex]->addControlPoint({800, 600});
            break;

        case 3: // Loop
            m_curves[m_activeCurveIndex]->addControlPoint({400, 300});
            m_curves[m_activeCurveIndex]->addControlPoint({600, 200});
            m_curves[m_activeCurveIndex]->addControlPoint({600, 500});
            m_curves[m_activeCurveIndex]->addControlPoint({400, 400});
            break;
    }
}

void CurveEditor::drawUI() const {
    // Title
    DrawText("BEZIER CURVE EDITOR", 10, 10, 30, RAYWHITE);

    // Instructions
    DrawText("Drag control points to modify curve", 10, 50, 16, LIGHTGRAY);

    // Curve info
    if (!m_curves.empty() && m_activeCurveIndex >= 0) {
        auto& curve = m_curves[m_activeCurveIndex];
        DrawText(TextFormat("Control Points: %d",
                           static_cast<int>(curve->getControlPoints().size())),
                 10, 80, 16, GREEN);

        DrawText(TextFormat("Curve Length: %.1f px", curve->getCurveLength()),
                 10, 100, 16, GREEN);

        DrawText(TextFormat("Type: %s",
                           curve->getType() == CurveType::Cubic ? "Cubic" : "Quadratic"),
                 10, 120, 16, GREEN);
    }

    // Controls
    DrawText("Controls:", 10, 160, 18, YELLOW);
    DrawText("1-4: Load presets", 10, 185, 14, GRAY);
    DrawText("A: Toggle animation", 10, 205, 14, GRAY);
    DrawText("C: Clear curve", 10, 225, 14, GRAY);
    DrawText("R: Reset to default", 10, 245, 14, GRAY);

    // Presets
    DrawText("Presets:", 10, 285, 18, YELLOW);
    DrawText("1: Smile", 10, 310, 14, GRAY);
    DrawText("2: Frown", 10, 330, 14, GRAY);
    DrawText("3: S-Curve", 10, 350, 14, GRAY);
    DrawText("4: Loop", 10, 370, 14, GRAY);

    // Animation status
    if (m_animating) {
        DrawText(TextFormat("Animation: %.2f", m_animationT),
                 10, 410, 16, RED);
    }

    DrawFPS(m_screenWidth - 100, 10);
}

void CurveEditor::handleInput() {
    // Load presets
    if (IsKeyPressed(KEY_ONE)) loadPreset(0);
    if (IsKeyPressed(KEY_TWO)) loadPreset(1);
    if (IsKeyPressed(KEY_THREE)) loadPreset(2);
    if (IsKeyPressed(KEY_FOUR)) loadPreset(3);

    // Toggle animation
    if (IsKeyPressed(KEY_A)) {
        m_animating = !m_animating;
        m_animationT = 0.0f;
    }

    // Clear curve
    if (IsKeyPressed(KEY_C) && !m_curves.empty()) {
        m_curves[m_activeCurveIndex]->clearControlPoints();
    }

    // Reset to default
    if (IsKeyPressed(KEY_R)) {
        clearCurves();
        addCurve(CurveType::Cubic);
        m_curves[0]->addControlPoint({200, 400});
        m_curves[0]->addControlPoint({300, 200});
        m_curves[0]->addControlPoint({700, 200});
        m_curves[0]->addControlPoint({800, 400});
    }

    // Add control point on right click
    if (IsMouseButtonPressed(MOUSE_RIGHT_BUTTON) && !m_curves.empty()) {
        Vector2 mousePos = GetMousePosition();
        m_curves[m_activeCurveIndex]->addControlPoint(mousePos);
    }
}

} // namespace Bezier
