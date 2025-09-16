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
    Transform transform;
} yv_Renderable3D;
