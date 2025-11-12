#pragma once

#include "theme.hpp"
#include <raylib.h>

namespace UI {

class ColorPicker {
public:
    ColorPicker(Vector2 position, float size, const Theme& theme);

    void update(Vector2 mousePos, bool mousePressed);
    void draw() const;

    Color getSelectedColor() const { return m_selectedColor; }
    void setTheme(const Theme& theme) { m_theme = theme; }

private:
    Vector2 m_position;
    float m_size;
    Theme m_theme;
    Color m_selectedColor;

    Rectangle m_hueBar;
    Rectangle m_satValBox;

    float m_hue;          // 0-360
    float m_saturation;   // 0-1
    float m_value;        // 0-1

    bool m_draggingHue;
    bool m_draggingSatVal;

    Color hsvToRgb(float h, float s, float v) const;
};

} // namespace UI
