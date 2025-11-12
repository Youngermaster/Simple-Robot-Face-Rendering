#include "particle.hpp"

namespace Particles {

Particle::Particle(Vector2 pos, Vector2 vel, Color col, float lifeSpan, float particleSize, float grav)
    : position(pos)
    , velocity(vel)
    , color(col)
    , life(lifeSpan)
    , size(particleSize)
    , gravity(grav)
{
}

void Particle::update(float deltaTime) {
    if (life <= 0.0f) return;

    // Update physics
    velocity.y += gravity * deltaTime;
    position.x += velocity.x * deltaTime;
    position.y += velocity.y * deltaTime;

    // Decrease life
    life -= deltaTime;
    if (life < 0.0f) life = 0.0f;
}

void Particle::draw() const {
    if (life <= 0.0f) return;

    // Fade out as particle dies
    Color drawColor = color;
    drawColor.a = static_cast<unsigned char>(255.0f * life);

    // Draw particle with glow effect
    DrawCircleV(position, size + 2.0f, ColorAlpha(drawColor, 0.3f));
    DrawCircleV(position, size, drawColor);
}

} // namespace Particles
