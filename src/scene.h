#pragma once

#include <raylib.h>

// Scene
// -----
//
// A representation of a full Scene in the game.
typedef struct {
    Camera3D camera;
} yv_Scene;

// Main scene
// ---------
//
// Creates the main scene for the game.
yv_Scene yv_create_main_scene(void);
