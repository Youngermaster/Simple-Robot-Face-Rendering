#include "bezier_curve.hpp"
#include <cmath>

namespace Bezier {

BezierCurve::BezierCurve(CurveType type)
    : m_type(type)
    , m_selectedPoint(-1)
    , m_hoverPoint(-1)
{
}

void BezierCurve::addControlPoint(Vector2 point) {
    m_controlPoints.push_back(point);
}

void BezierCurve::clearControlPoints() {
    m_controlPoints.clear();
    m_selectedPoint = -1;
    m_hoverPoint = -1;
}

void BezierCurve::setControlPoint(int index, Vector2 point) {
    if (index >= 0 && index < static_cast<int>(m_controlPoints.size())) {
        m_controlPoints[index] = point;
    }
}

void BezierCurve::update(Vector2 mousePos, bool mousePressed) {
    m_hoverPoint = findHoverPoint(mousePos);

    if (mousePressed) {
        if (m_selectedPoint == -1 && m_hoverPoint != -1) {
            m_selectedPoint = m_hoverPoint;
        }
    } else {
        m_selectedPoint = -1;
    }

    // Drag selected point
    if (m_selectedPoint != -1) {
        setControlPoint(m_selectedPoint, mousePos);
    }
}

void BezierCurve::draw() const {
    if (m_controlPoints.size() < 2) return;

    // Draw control polygon (lines between control points)
    for (size_t i = 0; i < m_controlPoints.size() - 1; i++) {
        DrawLineEx(
            m_controlPoints[i],
            m_controlPoints[i + 1],
            2.0f,
            ColorAlpha(GRAY, 0.5f)
        );
    }

    // Draw the actual bezier curve
    int segments = 100;
    for (int i = 0; i < segments; i++) {
        float t1 = static_cast<float>(i) / static_cast<float>(segments);
        float t2 = static_cast<float>(i + 1) / static_cast<float>(segments);

        Vector2 p1 = getPointAt(t1);
        Vector2 p2 = getPointAt(t2);

        DrawLineEx(p1, p2, 3.0f, SKYBLUE);
    }

    // Draw control points
    for (size_t i = 0; i < m_controlPoints.size(); i++) {
        bool isSelected = (static_cast<int>(i) == m_selectedPoint);
        bool isHovered = (static_cast<int>(i) == m_hoverPoint);
        drawControlPoint(m_controlPoints[i], isSelected, isHovered);
    }
}

Vector2 BezierCurve::getPointAt(float t) const {
    if (m_controlPoints.empty()) return {0, 0};
    if (m_controlPoints.size() == 1) return m_controlPoints[0];

    // De Casteljau's algorithm (works for any number of control points)
    std::vector<Vector2> points = m_controlPoints;

    while (points.size() > 1) {
        std::vector<Vector2> newPoints;
        for (size_t i = 0; i < points.size() - 1; i++) {
            Vector2 p = {
                points[i].x + t * (points[i + 1].x - points[i].x),
                points[i].y + t * (points[i + 1].y - points[i].y)
            };
            newPoints.push_back(p);
        }
        points = newPoints;
    }

    return points[0];
}

float BezierCurve::getCurveLength() const {
    if (m_controlPoints.size() < 2) return 0.0f;

    float length = 0.0f;
    int segments = 100;

    Vector2 prevPoint = getPointAt(0.0f);
    for (int i = 1; i <= segments; i++) {
        float t = static_cast<float>(i) / static_cast<float>(segments);
        Vector2 point = getPointAt(t);

        float dx = point.x - prevPoint.x;
        float dy = point.y - prevPoint.y;
        length += std::sqrt(dx * dx + dy * dy);

        prevPoint = point;
    }

    return length;
}

int BezierCurve::findHoverPoint(Vector2 mousePos) const {
    const float hoverRadius = 15.0f;

    for (size_t i = 0; i < m_controlPoints.size(); i++) {
        float dx = mousePos.x - m_controlPoints[i].x;
        float dy = mousePos.y - m_controlPoints[i].y;
        float distance = std::sqrt(dx * dx + dy * dy);

        if (distance <= hoverRadius) {
            return static_cast<int>(i);
        }
    }

    return -1;
}

void BezierCurve::drawControlPoint(Vector2 point, bool isSelected, bool isHovered) const {
    Color color = GRAY;
    float radius = 8.0f;

    if (isSelected) {
        color = RED;
        radius = 12.0f;
    } else if (isHovered) {
        color = YELLOW;
        radius = 10.0f;
    }

    // Draw glow
    DrawCircleV(point, radius + 4.0f, ColorAlpha(color, 0.3f));

    // Draw point
    DrawCircleV(point, radius, color);
    DrawCircleV(point, radius - 2.0f, WHITE);
}

} // namespace Bezier
