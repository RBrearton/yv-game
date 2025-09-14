#include <catch2/catch_test_macros.hpp>

#include "yvium/components/transform.hpp"
#include "yvium/entities/entity.hpp"

using namespace yvium::entities;
using namespace yvium::components;

TEST_CASE("Entity creation and ID management", "[entity]") {
    Entity entity(42);

    REQUIRE(entity.getId() == 42);
}

TEST_CASE("Component addition and retrieval", "[entity]") {
    Entity entity(1);

    SECTION("Add and get component") {
        auto& transform = entity.addComponent<Transform>(Vector2{10.0f, 20.0f});

        REQUIRE(entity.hasComponent<Transform>());

        auto* retrievedTransform = entity.getComponent<Transform>();
        REQUIRE(retrievedTransform != nullptr);
        REQUIRE(retrievedTransform->position.x == 10.0f);
        REQUIRE(retrievedTransform->position.y == 20.0f);
    }

    SECTION("Get non-existent component") {
        auto* transform = entity.getComponent<Transform>();
        REQUIRE(transform == nullptr);
        REQUIRE_FALSE(entity.hasComponent<Transform>());
    }
}

TEST_CASE("Component removal", "[entity]") {
    Entity entity(1);

    entity.addComponent<Transform>();
    REQUIRE(entity.hasComponent<Transform>());

    bool removed = entity.removeComponent<Transform>();
    REQUIRE(removed);
    REQUIRE_FALSE(entity.hasComponent<Transform>());

    // Try to remove again.
    bool removedAgain = entity.removeComponent<Transform>();
    REQUIRE_FALSE(removedAgain);
}
