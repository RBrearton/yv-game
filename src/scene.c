#include "scene.h"

#include "raylib.h"

// Default camera values.
const Vector3 yv_DEFAULT_CAMERA_POSITION = {10, 10, 10};
const Vector3 yv_DEFAULT_CAMERA_TARGET = {0, 0, 0};
const Vector3 yv_DEFAULT_CAMERA_UP = {0, 1, 0};
const float yv_DEFAULT_CAMERA_FOVY = 45.0F;
const int yv_DEFAULT_CAMERA_PROJECTION = CAMERA_PERSPECTIVE;

// Default camera
// --------------
//
// Creates the default camera for the main scene.
Camera3D yv_DefaultCamera(void) {
    return (Camera3D){.position = yv_DEFAULT_CAMERA_POSITION,
                      .target = yv_DEFAULT_CAMERA_TARGET,
                      .up = yv_DEFAULT_CAMERA_UP,
                      .fovy = yv_DEFAULT_CAMERA_FOVY,
                      .projection = yv_DEFAULT_CAMERA_PROJECTION};
}

yv_Scene yv_CreateMainScene(void) {
    yv_Scene scene;
    scene.camera = yv_DefaultCamera();
    return scene;
}

void yv_AddActor(yv_Scene* scene, yv_Renderable3D renderable) {
    // The next actor id is equal to the max actor id + 1.
    size_t nextActorId = scene->maxActorId + 1;

    // Wrap the renderable in an actor.
    yv_Actor actor;
    actor.renderable = renderable;
    actor.id = nextActorId;

    // Add the actor to the scene.
    scene->actors[scene->numActors] = actor;
    scene->numActors++;
    scene->maxActorId = nextActorId;
}
