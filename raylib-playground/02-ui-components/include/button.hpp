#pragma once

#include "theme.hpp"
#include <raylib.h>
#include <string>
#include <functional>

namespace UI {

class Button {
public:
    Button(Rectangle bounds, const std::string& text, const Theme& theme);

    void update(Vector2 mousePos, bool mousePressed);
    void draw() const;

    bool wasClicked() const { return m_wasClicked; }
    void setOnClick(std::function<void()> callback) { m_onClick = callback; }

    void setTheme(const Theme& theme) { m_theme = theme; }

private:
    Rectangle m_bounds;
    std::string m_text;
    Theme m_theme;

    bool m_isHovered;
    bool m_wasClicked;
    std::function<void()> m_onClick;
};

} // namespace UI
