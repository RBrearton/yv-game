#include "player.h"

yv_Renderable3D yv_CreatePlayer(Transform transform) {
    return (yv_Renderable3D){
        .modelType = CAPSULE,
        .transform = transform,
    };
}
