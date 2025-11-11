#include "solar_system.hpp"

namespace SolarSystem {

SolarSystemScene::SolarSystemScene(int screenWidth, int screenHeight)
    : m_screenWidth(screenWidth)
    , m_screenHeight(screenHeight)
    , m_sunRadius(30.0f)
{
    m_sunPosition = {
        static_cast<float>(screenWidth) / 2.0f,
        static_cast<float>(screenHeight) / 2.0f
    };
}

void SolarSystemScene::update(float deltaTime) {
    for (auto& planet : m_planets) {
        planet->update(deltaTime);
    }
}

void SolarSystemScene::draw() const {
    ClearBackground(Color{10, 10, 20, 255}); // Dark space background

    // Draw sun with glow effect
    DrawCircleV(m_sunPosition, m_sunRadius + 10.0f, ColorAlpha(YELLOW, 0.2f));
    DrawCircleV(m_sunPosition, m_sunRadius + 5.0f, ColorAlpha(ORANGE, 0.4f));
    DrawCircleV(m_sunPosition, m_sunRadius, YELLOW);

    // Draw all planets
    for (const auto& planet : m_planets) {
        planet->draw(m_sunPosition);
    }

    // Draw UI
    DrawText("SOLAR SYSTEM SIMULATION", 10, 10, 20, RAYWHITE);
    DrawText("Press SPACE to pause", 10, 35, 16, LIGHTGRAY);
    DrawFPS(m_screenWidth - 100, 10);
}

void SolarSystemScene::addPlanet(float orbitRadius, float planetRadius, float orbitSpeed, Color color) {
    m_planets.push_back(
        std::make_unique<Planet>(orbitRadius, planetRadius, orbitSpeed, color)
    );
}

} // namespace SolarSystem
