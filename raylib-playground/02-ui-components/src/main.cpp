#include "button.hpp"
#include "color_picker.hpp"
#include "theme.hpp"
#include <raylib.h>
#include <vector>

int main() {
    const int screenWidth = 1200;
    const int screenHeight = 800;

    InitWindow(screenWidth, screenHeight, "UI Components - Raylib Example");
    SetTargetFPS(60);

    // Current theme
    UI::ThemeType currentTheme = UI::ThemeType::Dark;
    UI::Theme theme = UI::Theme::getTheme(currentTheme);

    // Create buttons for theme switching
    std::vector<UI::Button> themeButtons;
    const char* themeNames[] = {"Light", "Dark", "Ocean", "Forest"};
    for (int i = 0; i < 4; i++) {
        Rectangle bounds = {50.0f + i * 150.0f, 50.0f, 130.0f, 50.0f};
        themeButtons.emplace_back(bounds, themeNames[i], theme);
    }

    // Set theme button callbacks
    themeButtons[0].setOnClick([&]() {
        currentTheme = UI::ThemeType::Light;
        theme = UI::Theme::getTheme(currentTheme);
    });
    themeButtons[1].setOnClick([&]() {
        currentTheme = UI::ThemeType::Dark;
        theme = UI::Theme::getTheme(currentTheme);
    });
    themeButtons[2].setOnClick([&]() {
        currentTheme = UI::ThemeType::Ocean;
        theme = UI::Theme::getTheme(currentTheme);
    });
    themeButtons[3].setOnClick([&]() {
        currentTheme = UI::ThemeType::Forest;
        theme = UI::Theme::getTheme(currentTheme);
    });

    // Create color picker
    UI::ColorPicker colorPicker(Vector2{50, 150}, 300, theme);

    // Create action buttons
    UI::Button resetButton(Rectangle{700, 150, 200, 60}, "Reset Color", theme);
    UI::Button randomButton(Rectangle{700, 230, 200, 60}, "Random Theme", theme);
    UI::Button infoButton(Rectangle{700, 310, 200, 60}, "Show Info", theme);

    bool showInfo = false;

    resetButton.setOnClick([&]() {
        colorPicker = UI::ColorPicker(Vector2{50, 150}, 300, theme);
    });

    randomButton.setOnClick([&]() {
        currentTheme = static_cast<UI::ThemeType>(GetRandomValue(0, 3));
        theme = UI::Theme::getTheme(currentTheme);
    });

    infoButton.setOnClick([&]() {
        showInfo = !showInfo;
    });

    while (!WindowShouldClose()) {
        // Update theme for all UI elements
        for (auto& btn : themeButtons) {
            btn.setTheme(theme);
        }
        resetButton.setTheme(theme);
        randomButton.setTheme(theme);
        infoButton.setTheme(theme);
        colorPicker.setTheme(theme);

        // Update
        Vector2 mousePos = GetMousePosition();
        bool mousePressed = IsMouseButtonDown(MOUSE_LEFT_BUTTON);

        for (auto& btn : themeButtons) {
            btn.update(mousePos, mousePressed && IsMouseButtonPressed(MOUSE_LEFT_BUTTON));
        }

        resetButton.update(mousePos, mousePressed && IsMouseButtonPressed(MOUSE_LEFT_BUTTON));
        randomButton.update(mousePos, mousePressed && IsMouseButtonPressed(MOUSE_LEFT_BUTTON));
        infoButton.update(mousePos, mousePressed && IsMouseButtonPressed(MOUSE_LEFT_BUTTON));

        colorPicker.update(mousePos, mousePressed);

        // Draw
        BeginDrawing();
        ClearBackground(theme.background);

        // Draw title
        DrawText("UI COMPONENTS SHOWCASE", 50, 10, 30, theme.textPrimary);

        // Draw theme buttons
        for (auto& btn : themeButtons) {
            btn.draw();
        }

        // Draw color picker
        colorPicker.draw();

        // Draw action buttons
        resetButton.draw();
        randomButton.draw();
        infoButton.draw();

        // Draw sample boxes with picked color
        Color pickedColor = colorPicker.getSelectedColor();
        DrawRectangle(700, 400, 200, 100, pickedColor);
        DrawRectangleLinesEx(Rectangle{700, 400, 200, 100}, 2, theme.textPrimary);
        DrawText("Picked Color", 720, 420, 20, theme.textPrimary);

        // Draw gradient with picked color
        DrawRectangleGradientV(700, 520, 200, 100, pickedColor, ColorAlpha(pickedColor, 0.2f));
        DrawRectangleLinesEx(Rectangle{700, 520, 200, 100}, 2, theme.textPrimary);
        DrawText("Gradient", 750, 560, 20, theme.textPrimary);

        // Draw info panel
        if (showInfo) {
            Rectangle infoPanel = {950, 150, 200, 400};
            DrawRectangleRec(infoPanel, theme.foreground);
            DrawRectangleLinesEx(infoPanel, 2, theme.accent);

            DrawText("INFO", 1020, 170, 20, theme.textPrimary);
            DrawText("Controls:", 970, 210, 16, theme.textSecondary);
            DrawText("- Click theme", 970, 240, 14, theme.textSecondary);
            DrawText("  buttons to", 970, 260, 14, theme.textSecondary);
            DrawText("  switch", 970, 280, 14, theme.textSecondary);
            DrawText("- Drag color", 970, 310, 14, theme.textSecondary);
            DrawText("  picker to", 970, 330, 14, theme.textSecondary);
            DrawText("  select", 970, 350, 14, theme.textSecondary);
            DrawText("- Use action", 970, 380, 14, theme.textSecondary);
            DrawText("  buttons", 970, 400, 14, theme.textSecondary);
            DrawText("", 970, 430, 14, theme.textSecondary);
            DrawText("Theme:", 970, 460, 16, theme.textSecondary);
            DrawText(UI::Theme::getThemeName(currentTheme), 970, 485, 18, theme.accent);
        }

        DrawFPS(screenWidth - 100, 10);

        EndDrawing();
    }

    CloseWindow();
    return 0;
}
