#include "curve_editor.hpp"
#include <raylib.h>

int main() {
    const int screenWidth = 1200;
    const int screenHeight = 800;

    InitWindow(screenWidth, screenHeight, "Bezier Curves - Raylib Example");
    SetTargetFPS(60);

    Bezier::CurveEditor editor(screenWidth, screenHeight);

    while (!WindowShouldClose()) {
        // Update
        editor.update(GetFrameTime());

        // Draw
        BeginDrawing();
        editor.draw();
        EndDrawing();
    }

    CloseWindow();
    return 0;
}
