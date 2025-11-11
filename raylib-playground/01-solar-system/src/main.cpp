#include "solar_system.hpp"
#include <raylib.h>

int main() {
    const int screenWidth = 1000;
    const int screenHeight = 800;

    InitWindow(screenWidth, screenHeight, "Solar System - Raylib Example");
    SetTargetFPS(60);

    SolarSystem::SolarSystemScene scene(screenWidth, screenHeight);

    // Add planets (orbitRadius, planetRadius, orbitSpeed, color)
    // Mercury-like
    scene.addPlanet(60.0f, 6.0f, 2.0f, GRAY);

    // Venus-like
    scene.addPlanet(100.0f, 10.0f, 1.5f, ORANGE);

    // Earth-like
    scene.addPlanet(150.0f, 12.0f, 1.0f, BLUE);

    // Mars-like
    scene.addPlanet(200.0f, 8.0f, 0.8f, RED);

    // Jupiter-like
    scene.addPlanet(280.0f, 20.0f, 0.4f, Color{200, 160, 120, 255});

    // Saturn-like
    scene.addPlanet(350.0f, 18.0f, 0.3f, Color{220, 180, 140, 255});

    bool paused = false;

    while (!WindowShouldClose()) {
        // Update
        if (IsKeyPressed(KEY_SPACE)) {
            paused = !paused;
        }

        if (!paused) {
            scene.update(GetFrameTime());
        }

        // Draw
        BeginDrawing();
        scene.draw();

        if (paused) {
            DrawText("PAUSED", screenWidth / 2 - 50, screenHeight - 40, 20, RED);
        }

        EndDrawing();
    }

    CloseWindow();
    return 0;
}
