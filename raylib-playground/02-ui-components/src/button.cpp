#include "button.hpp"

namespace UI {

Button::Button(Rectangle bounds, const std::string& text, const Theme& theme)
    : m_bounds(bounds)
    , m_text(text)
    , m_theme(theme)
    , m_isHovered(false)
    , m_wasClicked(false)
{
}

void Button::update(Vector2 mousePos, bool mousePressed) {
    m_wasClicked = false;
    m_isHovered = CheckCollisionPointRec(mousePos, m_bounds);

    if (m_isHovered && mousePressed) {
        m_wasClicked = true;
        if (m_onClick) {
            m_onClick();
        }
    }
}

void Button::draw() const {
    // Determine button color based on state
    Color buttonColor = m_theme.primary;
    if (m_isHovered) {
        buttonColor = m_theme.secondary;
    }

    // Draw button shadow
    DrawRectangleRec(
        Rectangle{m_bounds.x + 4, m_bounds.y + 4, m_bounds.width, m_bounds.height},
        ColorAlpha(BLACK, 0.3f)
    );

    // Draw button background
    DrawRectangleRounded(m_bounds, 0.3f, 10, buttonColor);

    // Draw button border
    DrawRectangleRoundedLines(m_bounds, 0.3f, 10, m_theme.accent);

    // Calculate text position (centered)
    int fontSize = 20;
    int textWidth = MeasureText(m_text.c_str(), fontSize);
    int textX = static_cast<int>(m_bounds.x + (m_bounds.width - textWidth) / 2);
    int textY = static_cast<int>(m_bounds.y + (m_bounds.height - fontSize) / 2);

    // Draw text
    DrawText(m_text.c_str(), textX, textY, fontSize, m_theme.textPrimary);
}

} // namespace UI
