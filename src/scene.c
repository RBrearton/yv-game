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
Camera3D yv_default_camera(void) {
    return (Camera3D){.position = yv_DEFAULT_CAMERA_POSITION,
                      .target = yv_DEFAULT_CAMERA_TARGET,
                      .up = yv_DEFAULT_CAMERA_UP,
                      .fovy = yv_DEFAULT_CAMERA_FOVY,
                      .projection = yv_DEFAULT_CAMERA_PROJECTION};
}

yv_Scene yv_CreateMainScene(void) {
    yv_Scene scene;
    scene.camera = yv_default_camera();
    return scene;
}
