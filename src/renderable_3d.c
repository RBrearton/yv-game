#include "renderable_3d.h"

#include <raylib.h>
#include <raymath.h>

yv_Renderable3D yv_CreateCube(yv_CubeData cubeData) {
    return (yv_Renderable3D){.modelType = CUBE, .data = {.cubeData = cubeData}};
}

yv_Renderable3D yv_CreateCapsule(yv_CapsuleData capsuleData) {
    return (yv_Renderable3D){.modelType = CAPSULE, .data = {.capsuleData = capsuleData}};
}

void yv_Render(yv_Renderable3D renderable) {
    switch (renderable.modelType) {
        case CUBE:
            DrawCube(renderable.data.cubeData.position, renderable.data.cubeData.width,
                     renderable.data.cubeData.height, renderable.data.cubeData.depth,
                     renderable.data.cubeData.color);
            break;
        case CAPSULE:
            DrawCapsule(renderable.data.capsuleData.startPos, renderable.data.capsuleData.endPos,
                        renderable.data.capsuleData.radius, renderable.data.capsuleData.slices,
                        renderable.data.capsuleData.rings, renderable.data.capsuleData.color);
            break;
    }
}
