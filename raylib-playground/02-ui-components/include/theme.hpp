#pragma once

#include <raylib.h>
#include <string>

namespace UI {

enum class ThemeType {
    Light,
    Dark,
    Ocean,
    Forest
};

struct Theme {
    Color background;
    Color foreground;
    Color primary;
    Color secondary;
    Color accent;
    Color textPrimary;
    Color textSecondary;

    static Theme getTheme(ThemeType type);
    static const char* getThemeName(ThemeType type);
};

} // namespace UI
