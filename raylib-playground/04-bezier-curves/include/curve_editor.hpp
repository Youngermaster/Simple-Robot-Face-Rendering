#pragma once

#include "bezier_curve.hpp"
#include <vector>
#include <memory>

namespace Bezier {

class CurveEditor {
public:
    CurveEditor(int screenWidth, int screenHeight);

    void update(float deltaTime);
    void draw() const;

    void addCurve(CurveType type);
    void clearCurves();

    void loadPreset(int presetIndex);

private:
    int m_screenWidth;
    int m_screenHeight;
    std::vector<std::unique_ptr<BezierCurve>> m_curves;
    int m_activeCurveIndex;

    float m_animationT;
    bool m_animating;

    void drawUI() const;
    void handleInput();
};

} // namespace Bezier
