#pragma once

#include <raylib.h>
#include <vector>

namespace Bezier {

enum class CurveType {
    Quadratic,   // 3 control points
    Cubic        // 4 control points
};

class BezierCurve {
public:
    BezierCurve(CurveType type = CurveType::Cubic);

    void addControlPoint(Vector2 point);
    void clearControlPoints();
    void setControlPoint(int index, Vector2 point);

    void update(Vector2 mousePos, bool mousePressed);
    void draw() const;

    // Calculate point on curve at t (0.0 to 1.0)
    Vector2 getPointAt(float t) const;

    // Get all control points
    const std::vector<Vector2>& getControlPoints() const { return m_controlPoints; }

    // Calculate curve length (approximate)
    float getCurveLength() const;

    CurveType getType() const { return m_type; }
    void setType(CurveType type) { m_type = type; }

private:
    CurveType m_type;
    std::vector<Vector2> m_controlPoints;
    int m_selectedPoint;
    int m_hoverPoint;

    int findHoverPoint(Vector2 mousePos) const;
    void drawControlPoint(Vector2 point, bool isSelected, bool isHovered) const;
};

} // namespace Bezier
