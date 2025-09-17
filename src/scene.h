#pragma once

#include <raylib.h>

#include "renderable_3d.h"

// Actor
// ------
//
// A representation of an actor in the game.
typedef struct {
    yv_Renderable3D renderable;
    size_t id;
} yv_Actor;

// Scene
// -----
//
// A representation of a full Scene in the game.
typedef struct {
    // Every scene has a camera.
    Camera3D camera;

    // The renderables in the scene.
    yv_Actor* actors;

    // The current largest actor id in the scene.
    size_t maxActorId;

    // The number of renderables in the scene.
    size_t numActors;

    // The current capacity for renderables in the scene.
    size_t capacity;
} yv_Scene;

// Main scene
// ---------
//
// Creates the main scene for the game.
yv_Scene yv_CreateMainScene(void);

// Add renderable
// --------------
//
// Adds a renderable to the scene.
void yv_AddActor(yv_Scene* scene, yv_Renderable3D renderable);
