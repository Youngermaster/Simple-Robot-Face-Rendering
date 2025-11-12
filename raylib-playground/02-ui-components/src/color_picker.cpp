#include "color_picker.hpp"
#include <cmath>
#include <algorithm>

namespace UI {

ColorPicker::ColorPicker(Vector2 position, float size, const Theme& theme)
    : m_position(position)
    , m_size(size)
    , m_theme(theme)
    , m_hue(0.0f)
    , m_saturation(1.0f)
    , m_value(1.0f)
    , m_draggingHue(false)
    , m_draggingSatVal(false)
{
    // Define picker rectangles
    m_satValBox = Rectangle{position.x, position.y, size, size};
    m_hueBar = Rectangle{position.x + size + 20, position.y, 30, size};

    m_selectedColor = hsvToRgb(m_hue, m_saturation, m_value);
}

void ColorPicker::update(Vector2 mousePos, bool mousePressed) {
    if (mousePressed) {
        if (CheckCollisionPointRec(mousePos, m_satValBox)) {
            m_draggingSatVal = true;
        }
        if (CheckCollisionPointRec(mousePos, m_hueBar)) {
            m_draggingHue = true;
        }
    } else {
        m_draggingHue = false;
        m_draggingSatVal = false;
    }

    // Update saturation/value
    if (m_draggingSatVal) {
        m_saturation = std::clamp((mousePos.x - m_satValBox.x) / m_satValBox.width, 0.0f, 1.0f);
        m_value = 1.0f - std::clamp((mousePos.y - m_satValBox.y) / m_satValBox.height, 0.0f, 1.0f);
        m_selectedColor = hsvToRgb(m_hue, m_saturation, m_value);
    }

    // Update hue
    if (m_draggingHue) {
        float hueNormalized = std::clamp((mousePos.y - m_hueBar.y) / m_hueBar.height, 0.0f, 1.0f);
        m_hue = hueNormalized * 360.0f;
        m_selectedColor = hsvToRgb(m_hue, m_saturation, m_value);
    }
}

void ColorPicker::draw() const {
    // Draw saturation/value box
    for (int y = 0; y < static_cast<int>(m_satValBox.height); y++) {
        for (int x = 0; x < static_cast<int>(m_satValBox.width); x++) {
            float s = static_cast<float>(x) / m_satValBox.width;
            float v = 1.0f - (static_cast<float>(y) / m_satValBox.height);
            Color color = hsvToRgb(m_hue, s, v);
            DrawPixel(
                static_cast<int>(m_satValBox.x + x),
                static_cast<int>(m_satValBox.y + y),
                color
            );
        }
    }

    // Draw saturation/value box border
    DrawRectangleLinesEx(m_satValBox, 2, m_theme.textPrimary);

    // Draw saturation/value selector
    Vector2 selectorPos = {
        m_satValBox.x + m_saturation * m_satValBox.width,
        m_satValBox.y + (1.0f - m_value) * m_satValBox.height
    };
    DrawCircleV(selectorPos, 8, WHITE);
    DrawCircleV(selectorPos, 6, m_selectedColor);
    DrawCircleLines(static_cast<int>(selectorPos.x), static_cast<int>(selectorPos.y), 8, BLACK);

    // Draw hue bar
    for (int y = 0; y < static_cast<int>(m_hueBar.height); y++) {
        float h = (static_cast<float>(y) / m_hueBar.height) * 360.0f;
        Color color = hsvToRgb(h, 1.0f, 1.0f);
        DrawRectangle(
            static_cast<int>(m_hueBar.x),
            static_cast<int>(m_hueBar.y + y),
            static_cast<int>(m_hueBar.width),
            1,
            color
        );
    }

    // Draw hue bar border
    DrawRectangleLinesEx(m_hueBar, 2, m_theme.textPrimary);

    // Draw hue selector
    float hueY = m_hueBar.y + (m_hue / 360.0f) * m_hueBar.height;
    DrawRectangle(
        static_cast<int>(m_hueBar.x - 3),
        static_cast<int>(hueY - 3),
        static_cast<int>(m_hueBar.width + 6),
        6,
        WHITE
    );
    DrawRectangle(
        static_cast<int>(m_hueBar.x - 2),
        static_cast<int>(hueY - 2),
        static_cast<int>(m_hueBar.width + 4),
        4,
        BLACK
    );

    // Draw color preview
    Rectangle previewBox = {
        m_position.x,
        m_position.y + m_size + 20,
        m_size + m_hueBar.width + 20,
        50
    };
    DrawRectangleRec(previewBox, m_selectedColor);
    DrawRectangleLinesEx(previewBox, 2, m_theme.textPrimary);

    // Draw color info
    DrawText(
        TextFormat("RGB: (%d, %d, %d)",
                   m_selectedColor.r,
                   m_selectedColor.g,
                   m_selectedColor.b),
        static_cast<int>(previewBox.x + 10),
        static_cast<int>(previewBox.y + 15),
        16,
        m_theme.textPrimary
    );
}

Color ColorPicker::hsvToRgb(float h, float s, float v) const {
    float c = v * s;
    float x = c * (1.0f - std::fabs(std::fmod(h / 60.0f, 2.0f) - 1.0f));
    float m = v - c;

    float r, g, b;

    if (h < 60.0f) {
        r = c; g = x; b = 0;
    } else if (h < 120.0f) {
        r = x; g = c; b = 0;
    } else if (h < 180.0f) {
        r = 0; g = c; b = x;
    } else if (h < 240.0f) {
        r = 0; g = x; b = c;
    } else if (h < 300.0f) {
        r = x; g = 0; b = c;
    } else {
        r = c; g = 0; b = x;
    }

    return Color{
        static_cast<unsigned char>((r + m) * 255),
        static_cast<unsigned char>((g + m) * 255),
        static_cast<unsigned char>((b + m) * 255),
        255
    };
}

} // namespace UI
