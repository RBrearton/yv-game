#pragma once

#include <raylib.h>

// Model type
// ----------
//
// A representation of a 3D model.
typedef enum {
    CAPSULE,
} yv_ModelType;

// Renderable3D
// ------------
//
// A representation of a 3D renderable object.
typedef struct {
    yv_ModelType modelType;
    Vector3 position;
    Vector3 rotation;
    Vector3 scale;
} yv_Renderable3D;
