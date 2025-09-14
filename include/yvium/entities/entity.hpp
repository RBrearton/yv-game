#pragma once

#include <cstdint>
#include <memory>
#include <typeindex>
#include <unordered_map>

namespace yvium::entities {

using EntityId = uint32_t;
constexpr EntityId INVALID_ENTITY_ID = 0;

/**
 * @brief Base component class for the Entity Component System.
 */
class Component {
   public:
    virtual ~Component() = default;
};

/**
 * @brief Entity class that can hold multiple components.
 *
 * This follows the Entity Component System (ECS) pattern which is
 * ideal for game development as it provides flexibility and performance.
 */
class Entity {
   public:
    explicit Entity(EntityId id) : m_id(id) {}
    ~Entity() = default;

    /**
     * @brief Get the entity's unique identifier.
     * @return The entity ID.
     */
    EntityId getId() const { return m_id; }

    /**
     * @brief Add a component to this entity.
     * @tparam T The component type.
     * @tparam Args Constructor argument types.
     * @param args Constructor arguments.
     * @return Reference to the added component.
     */
    template <typename T, typename... Args>
    T& addComponent(Args&&... args) {
        static_assert(std::is_base_of_v<Component, T>, "T must derive from Component");

        auto component = std::make_unique<T>(std::forward<Args>(args)...);
        T* componentPtr = component.get();
        m_components[std::type_index(typeid(T))] = std::move(component);
        return *componentPtr;
    }

    /**
     * @brief Get a component of the specified type.
     * @tparam T The component type.
     * @return Pointer to the component, or nullptr if not found.
     */
    template <typename T>
    T* getComponent() {
        static_assert(std::is_base_of_v<Component, T>, "T must derive from Component");

        auto it = m_components.find(std::type_index(typeid(T)));
        if (it != m_components.end()) {
            return static_cast<T*>(it->second.get());
        }
        return nullptr;
    }

    /**
     * @brief Check if the entity has a component of the specified type.
     * @tparam T The component type.
     * @return true if the component exists, false otherwise.
     */
    template <typename T>
    bool hasComponent() const {
        static_assert(std::is_base_of_v<Component, T>, "T must derive from Component");
        return m_components.find(std::type_index(typeid(T))) != m_components.end();
    }

    /**
     * @brief Remove a component of the specified type.
     * @tparam T The component type.
     * @return true if the component was removed, false if it didn't exist.
     */
    template <typename T>
    bool removeComponent() {
        static_assert(std::is_base_of_v<Component, T>, "T must derive from Component");
        return m_components.erase(std::type_index(typeid(T))) > 0;
    }

   private:
    EntityId m_id;
    std::unordered_map<std::type_index, std::unique_ptr<Component>> m_components;
};

}  // namespace yvium::entities
