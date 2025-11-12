#include "particle_emitter.hpp"
#include <raylib.h>
#include <memory>

int main() {
    const int screenWidth = 1200;
    const int screenHeight = 800;

    InitWindow(screenWidth, screenHeight, "Particle System - Raylib Example");
    SetTargetFPS(60);

    // Create emitter at center of screen
    Vector2 emitterPos = {static_cast<float>(screenWidth) / 2.0f, static_cast<float>(screenHeight) / 2.0f};
    auto emitter = std::make_unique<Particles::ParticleEmitter>(emitterPos, Particles::EmitterType::Fountain);

    Particles::EmitterType currentType = Particles::EmitterType::Fountain;
    bool followMouse = true;

    while (!WindowShouldClose()) {
        // Update emitter position to follow mouse
        if (followMouse) {
            emitterPos = GetMousePosition();
            emitter->setPosition(emitterPos);
        }

        // Handle input
        if (IsKeyPressed(KEY_ONE)) {
            currentType = Particles::EmitterType::Fountain;
            emitter->setType(currentType);
            emitter->clear();
        }
        if (IsKeyPressed(KEY_TWO)) {
            currentType = Particles::EmitterType::Explosion;
            emitter->setType(currentType);
            emitter->clear();
        }
        if (IsKeyPressed(KEY_THREE)) {
            currentType = Particles::EmitterType::Fire;
            emitter->setType(currentType);
            emitter->clear();
        }
        if (IsKeyPressed(KEY_FOUR)) {
            currentType = Particles::EmitterType::Snow;
            emitter->setType(currentType);
            emitter->clear();
        }
        if (IsKeyPressed(KEY_FIVE)) {
            currentType = Particles::EmitterType::Confetti;
            emitter->setType(currentType);
            emitter->clear();
        }

        if (IsKeyPressed(KEY_M)) {
            followMouse = !followMouse;
        }

        if (IsKeyPressed(KEY_C)) {
            emitter->clear();
        }

        // Manual emission with mouse click
        if (IsMouseButtonPressed(MOUSE_LEFT_BUTTON)) {
            if (currentType == Particles::EmitterType::Explosion) {
                emitter->emit(100);
            } else if (currentType == Particles::EmitterType::Confetti) {
                emitter->emit(50);
            } else {
                emitter->emit(20);
            }
        }

        // Update
        emitter->update(GetFrameTime());

        // Draw
        BeginDrawing();
        ClearBackground(Color{15, 15, 25, 255});

        emitter->draw();

        // Draw UI
        DrawText("PARTICLE SYSTEM", 10, 10, 30, RAYWHITE);

        DrawText("Controls:", 10, 50, 20, LIGHTGRAY);
        DrawText("1-5: Change particle type", 10, 75, 16, GRAY);
        DrawText("M: Toggle mouse follow", 10, 95, 16, GRAY);
        DrawText("C: Clear particles", 10, 115, 16, GRAY);
        DrawText("Click: Emit burst", 10, 135, 16, GRAY);

        // Show current type
        const char* typeNames[] = {"Fountain", "Explosion", "Fire", "Snow", "Confetti"};
        DrawText(TextFormat("Type: %s", typeNames[static_cast<int>(currentType)]),
                 10, 165, 18, YELLOW);

        DrawText(TextFormat("Particles: %d", emitter->getParticleCount()),
                 10, 190, 18, GREEN);

        DrawText(TextFormat("Mouse Follow: %s", followMouse ? "ON" : "OFF"),
                 10, 215, 18, followMouse ? GREEN : RED);

        DrawFPS(screenWidth - 100, 10);

        EndDrawing();
    }

    CloseWindow();
    return 0;
}
