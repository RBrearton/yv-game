#include <iostream>

#include "yvium/core/game.hpp"

int main() {
    auto& game = yvium::core::Game::getInstance();

    if (!game.initialize()) {
        std::cerr << "Failed to initialize game!" << std::endl;
        return -1;
    }

    game.run();
    game.shutdown();

    return 0;
}
