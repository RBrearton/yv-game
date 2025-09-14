#pragma once

namespace yvium::systems {

/**
 * @brief Base class for all game systems.
 *
 * Systems contain the logic that operates on entities with specific components.
 * This follows the Entity Component System (ECS) architecture pattern.
 */
class System {
   public:
    virtual ~System() = default;

    /**
     * @brief Initialize the system.
     * Called once when the system is added to the game.
     */
    virtual void initialize() {}

    /**
     * @brief Update the system.
     * Called every frame with the delta time.
     * @param deltaTime Time elapsed since the last frame in seconds.
     */
    virtual void update(float deltaTime) = 0;

    /**
     * @brief Shutdown the system.
     * Called when the system is being removed or the game is shutting down.
     */
    virtual void shutdown() {}

   protected:
    bool m_active{true};  ///< Whether this system is currently active.

   public:
    /**
     * @brief Set the active state of this system.
     * @param active The new active state.
     */
    void setActive(bool active) { m_active = active; }

    /**
     * @brief Check if this system is active.
     * @return true if the system is active, false otherwise.
     */
    bool isActive() const { return m_active; }
};

}  // namespace yvium::systems
