#include "planet.hpp"
#include <cmath>

namespace SolarSystem {

Planet::Planet(float orbitRadius, float planetRadius, float orbitSpeed, Color color)
    : m_orbitRadius(orbitRadius)
    , m_planetRadius(planetRadius)
    , m_orbitSpeed(orbitSpeed)
    , m_currentAngle(0.0f)
    , m_color(color)
{
}

void Planet::update(float deltaTime) {
    m_currentAngle += m_orbitSpeed * deltaTime;

    // Keep angle in range [0, 2*PI]
    if (m_currentAngle > 2.0f * PI) {
        m_currentAngle -= 2.0f * PI;
    }
}

void Planet::draw(Vector2 centerPos) const {
    // Draw orbit path (thin circle)
    DrawCircleLines(
        static_cast<int>(centerPos.x),
        static_cast<int>(centerPos.y),
        m_orbitRadius,
        ColorAlpha(GRAY, 0.3f)
    );

    // Calculate planet position
    Vector2 planetPos = getPosition(centerPos);

    // Draw planet
    DrawCircleV(planetPos, m_planetRadius, m_color);

    // Draw a subtle glow effect
    DrawCircleV(planetPos, m_planetRadius + 2.0f, ColorAlpha(m_color, 0.3f));
}

Vector2 Planet::getPosition(Vector2 centerPos) const {
    return {
        centerPos.x + m_orbitRadius * std::cos(m_currentAngle),
        centerPos.y + m_orbitRadius * std::sin(m_currentAngle)
    };
}

} // namespace SolarSystem
