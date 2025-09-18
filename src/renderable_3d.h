#pragma once

#include <raylib.h>
#include <stddef.h>

// The different kinds of models that can be rendered.
typedef enum {
    CAPSULE,
    CUBE,
} yv_ModelType;

// The data required to render a cube.
typedef struct {
    float width;
    float height;
    float depth;
    Vector3 position;
    Color color;
} yv_CubeData;

// The data required to render a capsule.
typedef struct {
    Vector3 startPos;
    Vector3 endPos;
    float radius;
    int slices;
    int rings;
    Color color;
} yv_CapsuleData;

// The union of all data that could be required to render a model.
typedef union {
    yv_CubeData cubeData;
    yv_CapsuleData capsuleData;
} yv_Renderable3DData;

// A representation of a 3D renderable object.
typedef struct {
    // The type of model that we use to render the object.
    yv_ModelType modelType;

    // The data required to render this model type.
    yv_Renderable3DData data;
} yv_Renderable3D;

// Creates a cube renderable.
yv_Renderable3D yv_CreateCube(yv_CubeData cubeData);

// Creates a capsule renderable.
yv_Renderable3D yv_CreateCapsule(yv_CapsuleData capsuleData);

// Renders a renderable.
void yv_Render(yv_Renderable3D renderable);
