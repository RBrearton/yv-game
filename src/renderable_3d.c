#include "renderable_3d.h"

yv_Renderable3D yv_CreateCube(Transform transform) {
    return (yv_Renderable3D){.modelType = CUBE, .transform = transform};
}

yv_Renderable3D yv_CreateCapsule(Transform transform) {
    return (yv_Renderable3D){.modelType = CAPSULE, .transform = transform};
}
