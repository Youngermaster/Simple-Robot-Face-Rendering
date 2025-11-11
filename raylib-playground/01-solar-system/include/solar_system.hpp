#pragma once

#include "planet.hpp"
#include <vector>
#include <memory>

namespace SolarSystem {

class SolarSystemScene {
public:
    SolarSystemScene(int screenWidth, int screenHeight);
    ~SolarSystemScene() = default;

    void update(float deltaTime);
    void draw() const;

    void addPlanet(float orbitRadius, float planetRadius, float orbitSpeed, Color color);

private:
    int m_screenWidth;
    int m_screenHeight;
    Vector2 m_sunPosition;
    float m_sunRadius;

    std::vector<std::unique_ptr<Planet>> m_planets;
};

} // namespace SolarSystem
