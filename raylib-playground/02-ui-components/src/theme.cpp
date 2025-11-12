#include "theme.hpp"

namespace UI {

Theme Theme::getTheme(ThemeType type) {
    switch (type) {
        case ThemeType::Light:
            return Theme{
                .background = Color{240, 240, 245, 255},
                .foreground = Color{255, 255, 255, 255},
                .primary = Color{100, 120, 220, 255},
                .secondary = Color{150, 170, 240, 255},
                .accent = Color{255, 100, 120, 255},
                .textPrimary = Color{30, 30, 30, 255},
                .textSecondary = Color{100, 100, 100, 255}
            };

        case ThemeType::Dark:
            return Theme{
                .background = Color{20, 20, 25, 255},
                .foreground = Color{40, 40, 50, 255},
                .primary = Color{100, 150, 255, 255},
                .secondary = Color{70, 100, 180, 255},
                .accent = Color{255, 100, 150, 255},
                .textPrimary = Color{240, 240, 240, 255},
                .textSecondary = Color{160, 160, 160, 255}
            };

        case ThemeType::Ocean:
            return Theme{
                .background = Color{15, 30, 50, 255},
                .foreground = Color{25, 50, 80, 255},
                .primary = Color{50, 150, 200, 255},
                .secondary = Color{30, 100, 150, 255},
                .accent = Color{100, 220, 255, 255},
                .textPrimary = Color{230, 240, 255, 255},
                .textSecondary = Color{150, 180, 200, 255}
            };

        case ThemeType::Forest:
            return Theme{
                .background = Color{25, 35, 25, 255},
                .foreground = Color{40, 55, 40, 255},
                .primary = Color{80, 150, 80, 255},
                .secondary = Color{60, 110, 60, 255},
                .accent = Color{150, 220, 100, 255},
                .textPrimary = Color{230, 240, 220, 255},
                .textSecondary = Color{160, 180, 150, 255}
            };

        default:
            return getTheme(ThemeType::Dark);
    }
}

const char* Theme::getThemeName(ThemeType type) {
    switch (type) {
        case ThemeType::Light: return "Light";
        case ThemeType::Dark: return "Dark";
        case ThemeType::Ocean: return "Ocean";
        case ThemeType::Forest: return "Forest";
        default: return "Unknown";
    }
}

} // namespace UI
