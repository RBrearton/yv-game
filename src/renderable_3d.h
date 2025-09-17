#pragma once

#include <raylib.h>
#include <stddef.h>

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
    // The renderable's unique identifier.
    size_t id;

    // The type of model that we use to render the object.
    yv_ModelType modelType;

    // The transform of the object.
    Transform transform;
} yv_Renderable3D;
