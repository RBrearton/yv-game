#include "camera_utils.h"

#include <raylib.h>
#include <raymath.h>

void yv_UpdateFreeCamera(Camera* camera) {
    // Cache the original position/target.
    Vector3 oldPosition = camera->position;
    Vector3 oldTarget = camera->target;

    // Update camera normally using the inbuilt raylib function.
    UpdateCamera(camera, CAMERA_FREE);

    // If shift is held, amplify the movement.
    if (IsKeyDown(KEY_LEFT_SHIFT) || IsKeyDown(KEY_RIGHT_SHIFT)) {
        Vector3 movement = Vector3Subtract(camera->position, oldPosition);
        Vector3 targetMovement = Vector3Subtract(camera->target, oldTarget);

        // Apply additional movement (2x more for 3x total speed).
        camera->position = Vector3Add(camera->position, Vector3Scale(movement, 2.0f));
        camera->target = Vector3Add(camera->target, Vector3Scale(targetMovement, 2.0f));
    }
}
