#pragma once

#include "particle.hpp"
#include <vector>
#include <memory>

namespace Particles {

enum class EmitterType {
    Fountain,
    Explosion,
    Fire,
    Snow,
    Confetti
};

class ParticleEmitter {
public:
    ParticleEmitter(Vector2 position, EmitterType type);

    void update(float deltaTime);
    void draw() const;

    void emit(int count);
    void setPosition(Vector2 pos) { m_position = pos; }
    void setType(EmitterType type) { m_type = type; }

    int getParticleCount() const { return static_cast<int>(m_particles.size()); }
    void clear() { m_particles.clear(); }

private:
    Vector2 m_position;
    EmitterType m_type;
    std::vector<Particle> m_particles;
    float m_emissionTimer;

    void emitSingle();
    Particle createParticle() const;
};

} // namespace Particles
