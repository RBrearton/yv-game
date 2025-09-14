#include <raylib.h>

int main(void) {
    // Initialize the window.
    const int screenWidth = 1280;
    const int screenHeight = 720;
    const int target_fps = 60;
    InitWindow(screenWidth, screenHeight, "yv Game");

    SetTargetFPS(target_fps);

    // Main game loop.
    while (!WindowShouldClose()) {
        // Update game logic here.

        // Draw everything.
        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Welcome to yv!", screenWidth / 2 - 120, screenHeight / 2 - 20, 20, DARKGRAY);
        DrawText("Press ESC to exit", screenWidth / 2 - 80, screenHeight / 2 + 20, 16, GRAY);
        EndDrawing();
    }

    // Cleanup.
    CloseWindow();

    return 0;
}