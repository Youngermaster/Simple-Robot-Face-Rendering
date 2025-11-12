#pragma once

#include <raylib.h>

namespace Particles {

struct Particle {
    Vector2 position;
    Vector2 velocity;
    Color color;
    float life;        // 0.0 to 1.0
    float size;
    float gravity;

    Particle(Vector2 pos, Vector2 vel, Color col, float lifeSpan, float particleSize, float grav = 200.0f);

    void update(float deltaTime);
    void draw() const;

    bool isAlive() const { return life > 0.0f; }
};

} // namespace Particles
