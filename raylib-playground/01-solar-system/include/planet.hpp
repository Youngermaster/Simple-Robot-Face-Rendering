#pragma once

#include <raylib.h>

namespace SolarSystem {

class Planet {
public:
    Planet(float orbitRadius, float planetRadius, float orbitSpeed, Color color);

    void update(float deltaTime);
    void draw(Vector2 centerPos) const;

    // Getters
    Vector2 getPosition(Vector2 centerPos) const;
    float getOrbitRadius() const { return m_orbitRadius; }

private:
    float m_orbitRadius;      // Distance from the sun
    float m_planetRadius;     // Size of the planet
    float m_orbitSpeed;       // Radians per second
    float m_currentAngle;     // Current position in orbit (radians)
    Color m_color;
};

} // namespace SolarSystem
