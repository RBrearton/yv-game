#pragma once

#include <raylib.h>

#include "yvium/entities/entity.hpp"

namespace yvium::components {

/**
 * @brief Transform component that represents position, rotation, and scale.
 *
 * This is a fundamental component that most game entities will have.
 */
class Transform : public yvium::entities::Component {
   public:
    Transform(Vector2 position = {0.0f, 0.0f}, float rotation = 0.0f, Vector2 scale = {1.0f, 1.0f})
        : position(position), rotation(rotation), scale(scale) {}

    Vector2 position;  ///< Position in 2D space.
    float rotation;    ///< Rotation in degrees.
    Vector2 scale;     ///< Scale factors for X and Y axes.

    /**
     * @brief Get the transformation matrix for this transform.
     * @return The transformation matrix.
     */
    Matrix getMatrix() const {
        Matrix translation = MatrixTranslate(position.x, position.y, 0.0f);
        Matrix rotationMat = MatrixRotateZ(rotation * DEG2RAD);
        Matrix scaleMat = MatrixScale(scale.x, scale.y, 1.0f);

        return MatrixMultiply(MatrixMultiply(scaleMat, rotationMat), translation);
    }
};

/**
 * @brief Velocity component for entities that move.
 */
class Velocity : public yvium::entities::Component {
   public:
    explicit Velocity(Vector2 velocity = {0.0f, 0.0f}) : velocity(velocity) {}

    Vector2 velocity;  ///< Velocity vector in units per second.
};

}  // namespace yvium::components
