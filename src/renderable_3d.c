#include "renderable_3d.h"

yv_Renderable3D yv_CreateCube(size_t renderable_id, Transform transform) {
    return (yv_Renderable3D){.id = renderable_id, .modelType = CUBE, .transform = transform};
}

yv_Renderable3D yv_CreateCapsule(size_t renderable_id, Transform transform) {
    return (yv_Renderable3D){.id = renderable_id, .modelType = CAPSULE, .transform = transform};
}
