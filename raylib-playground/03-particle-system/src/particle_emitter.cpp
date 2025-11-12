#include "particle_emitter.hpp"
#include <cmath>

namespace Particles {

ParticleEmitter::ParticleEmitter(Vector2 position, EmitterType type)
    : m_position(position)
    , m_type(type)
    , m_emissionTimer(0.0f)
{
    m_particles.reserve(1000);
}

void ParticleEmitter::update(float deltaTime) {
    // Update existing particles
    auto it = m_particles.begin();
    while (it != m_particles.end()) {
        it->update(deltaTime);
        if (!it->isAlive()) {
            it = m_particles.erase(it);
        } else {
            ++it;
        }
    }

    // Auto-emit for continuous effects
    m_emissionTimer += deltaTime;
    float emissionRate = 0.05f; // seconds between emissions

    if (m_type == EmitterType::Fire || m_type == EmitterType::Snow) {
        if (m_emissionTimer >= emissionRate) {
            int emitCount = (m_type == EmitterType::Fire) ? 5 : 3;
            emit(emitCount);
            m_emissionTimer = 0.0f;
        }
    }
}

void ParticleEmitter::draw() const {
    for (const auto& particle : m_particles) {
        particle.draw();
    }

    // Draw emitter position marker
    DrawCircleV(m_position, 5, ColorAlpha(WHITE, 0.5f));
}

void ParticleEmitter::emit(int count) {
    for (int i = 0; i < count; i++) {
        emitSingle();
    }
}

void ParticleEmitter::emitSingle() {
    if (m_particles.size() >= 1000) return; // Max particle limit

    m_particles.push_back(createParticle());
}

Particle ParticleEmitter::createParticle() const {
    Vector2 velocity;
    Color color;
    float life;
    float size;
    float gravity;

    switch (m_type) {
        case EmitterType::Fountain: {
            float angle = GetRandomValue(-30, 30) * DEG2RAD;
            float speed = GetRandomValue(150, 300);
            velocity = {
                std::sin(angle) * speed,
                -std::cos(angle) * speed
            };
            color = Color{
                static_cast<unsigned char>(GetRandomValue(100, 255)),
                static_cast<unsigned char>(GetRandomValue(150, 255)),
                255,
                255
            };
            life = GetRandomValue(10, 20) / 10.0f;
            size = GetRandomValue(3, 7);
            gravity = 200.0f;
            break;
        }

        case EmitterType::Explosion: {
            float angle = GetRandomValue(0, 360) * DEG2RAD;
            float speed = GetRandomValue(100, 400);
            velocity = {
                std::cos(angle) * speed,
                std::sin(angle) * speed
            };
            color = Color{
                255,
                static_cast<unsigned char>(GetRandomValue(100, 200)),
                static_cast<unsigned char>(GetRandomValue(0, 100)),
                255
            };
            life = GetRandomValue(5, 15) / 10.0f;
            size = GetRandomValue(4, 10);
            gravity = 100.0f;
            break;
        }

        case EmitterType::Fire: {
            float angle = GetRandomValue(-20, 20) * DEG2RAD;
            float speed = GetRandomValue(30, 80);
            velocity = {
                std::sin(angle) * speed,
                -std::abs(std::cos(angle)) * speed - 50.0f
            };
            int colorChoice = GetRandomValue(0, 2);
            if (colorChoice == 0) {
                color = Color{255, static_cast<unsigned char>(GetRandomValue(100, 200)), 0, 255};
            } else if (colorChoice == 1) {
                color = Color{255, static_cast<unsigned char>(GetRandomValue(200, 255)), 0, 255};
            } else {
                color = Color{255, 50, 0, 255};
            }
            life = GetRandomValue(8, 15) / 10.0f;
            size = GetRandomValue(3, 8);
            gravity = -50.0f; // Negative gravity for rising
            break;
        }

        case EmitterType::Snow: {
            float angle = GetRandomValue(-10, 10) * DEG2RAD;
            float speed = GetRandomValue(20, 50);
            velocity = {
                std::sin(angle) * speed,
                speed
            };
            color = Color{255, 255, 255, 255};
            life = GetRandomValue(30, 50) / 10.0f;
            size = GetRandomValue(2, 5);
            gravity = 20.0f;
            break;
        }

        case EmitterType::Confetti: {
            float angle = GetRandomValue(-45, 45) * DEG2RAD;
            float speed = GetRandomValue(200, 400);
            velocity = {
                std::sin(angle) * speed,
                -std::cos(angle) * speed
            };
            color = Color{
                static_cast<unsigned char>(GetRandomValue(100, 255)),
                static_cast<unsigned char>(GetRandomValue(100, 255)),
                static_cast<unsigned char>(GetRandomValue(100, 255)),
                255
            };
            life = GetRandomValue(15, 25) / 10.0f;
            size = GetRandomValue(3, 6);
            gravity = 250.0f;
            break;
        }

        default:
            velocity = {0, -100};
            color = WHITE;
            life = 1.0f;
            size = 5.0f;
            gravity = 200.0f;
    }

    return Particle(m_position, velocity, color, life, size, gravity);
}

} // namespace Particles
