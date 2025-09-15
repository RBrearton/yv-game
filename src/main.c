#include <raylib.h>

#include "scene.h"

int main(void) {
    // Initialize the window.
    const int screenWidth = 1280;
    const int screenHeight = 720;
    const int target_fps = 60;
    InitWindow(screenWidth, screenHeight, "yv Game");
    SetTargetFPS(target_fps);

    // Initialize the main scene.
    yv_Scene main_scene = yv_create_main_scene();
    Vector3 cubePosition = {0.0f, 0.0f, 0.0f};

    // Main game loop.
    while (!WindowShouldClose()) {
        // Update game logic here.
        UpdateCamera(&main_scene.camera, CAMERA_FREE);

        // Start drawing this frame.
        BeginDrawing();
        ClearBackground(RAYWHITE);

        // Draw all 3d objects.
        BeginMode3D(main_scene.camera);
        DrawCube(cubePosition, 2.0f, 2.0f, 2.0f, RED);
        DrawCubeWires(cubePosition, 2.0f, 2.0f, 2.0f, MAROON);
        DrawGrid(10, 1.0f);
        EndMode3D();

        // Draw the UI.
        DrawText("Welcome to yv!", screenWidth / 2 - 120, screenHeight / 2 - 20, 20, DARKGRAY);
        DrawText("Press ESC to exit", screenWidth / 2 - 80, screenHeight / 2 + 20, 16, GRAY);

        // Finish drawing this frame.
        EndDrawing();
    }

    // Cleanup.
    CloseWindow();

    return 0;
}
