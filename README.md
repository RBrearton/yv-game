# Yvium Game Engine

A modern C++20 game engine built with raylib, featuring a clean Entity Component System (ECS) architecture.

## 🚀 Features

- **Modern C++20**: Leveraging the latest C++ features for clean, efficient code
- **Entity Component System**: Flexible and performant ECS architecture
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Modern Build System**: CMake with FetchContent for dependency management
- **Code Quality**: Integrated linting, formatting, and testing
- **Documentation**: Comprehensive docs with Doxygen-style comments

## 📁 Project Structure

```
yvium_rlib/
├── assets/                    # Game assets
│   ├── textures/             # Image files and sprites
│   ├── sounds/               # Audio files
│   ├── fonts/                # Font files
│   ├── shaders/              # Shader programs
│   └── data/                 # Game data files
├── build/                    # Build output (auto-generated)
├── cmake/                    # Custom CMake modules
├── docs/                     # Documentation
├── external/                 # Third-party dependencies
├── include/yvium/           # Public headers
│   ├── core/                # Core engine systems
│   ├── entities/            # Entity and component definitions
│   ├── systems/             # System implementations
│   ├── components/          # Component types
│   ├── scenes/              # Scene management
│   └── utils/               # Utility functions
├── scripts/                 # Build and utility scripts
├── src/                     # Source files
│   ├── game/               # Game logic implementation
│   │   ├── core/           # Core systems
│   │   ├── entities/       # Entity implementations
│   │   ├── systems/        # System implementations
│   │   ├── components/     # Component implementations
│   │   ├── scenes/         # Scene implementations
│   │   └── utils/          # Utility implementations
│   └── main.cpp            # Application entry point
├── tests/                   # Test suite
│   ├── unit/               # Unit tests
│   └── integration/        # Integration tests
└── tools/                   # Development tools
```

## 🛠️ Prerequisites

- **C++20 compatible compiler**:
  - GCC 10+ or Clang 12+ (Linux/macOS)
  - Visual Studio 2019+ or MSVC 19.28+ (Windows)
- **CMake 3.20+**
- **Git** (for dependency fetching)

### Optional Tools
- **clang-format** (for code formatting)
- **clangd** (for LSP support)
- **Doxygen** (for documentation generation)

## 🚀 Quick Start

### 1. Clone and Build

```bash
git clone <repository-url> yvium_rlib
cd yvium_rlib

# Build in release mode
./scripts/build.sh

# Or build in debug mode with tests
./scripts/build.sh --debug --tests
```

### 2. Run the Game

```bash
./build/bin/yvium-rlib
```

## 🔧 Development

### Building Manually

```bash
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . -j$(nproc)
```

### Building with Tests

```bash
mkdir build && cd build
cmake .. -DCMAKE_BUILD_TYPE=Debug -DBUILD_TESTS=ON
cmake --build . -j$(nproc)
ctest --output-on-failure
```

### Code Formatting

The project uses clang-format for consistent code style. Install it with:

```bash
brew install clang-format
```

#### Formatting Commands

```bash
# Check formatting without making changes
./scripts/format.sh --check

# Fix formatting issues automatically
./scripts/format.sh --fix

# Set up Git pre-commit hooks (recommended)
./scripts/setup-git-hooks.sh
```

#### IDE Integration

- **VS Code**: Automatic formatting on save is configured
- **Other editors**: Use the `.clang-format` file in the project root

### LSP Support

The project includes `.clangd` configuration for language server support in modern editors like VS Code, Neovim, and Emacs.

## 🏗️ Architecture

### Entity Component System (ECS)

The engine uses a flexible ECS architecture:

- **Entities**: Unique identifiers for game objects
- **Components**: Data containers (Transform, Velocity, Sprite, etc.)
- **Systems**: Logic processors that operate on entities with specific components

### Example Usage

```cpp
#include "yvium/entities/entity.hpp"
#include "yvium/components/transform.hpp"

// Create an entity
yvium::entities::Entity player(1);

// Add components
auto& transform = player.addComponent<yvium::components::Transform>(
    Vector2{100.0f, 100.0f}  // position
);

auto& velocity = player.addComponent<yvium::components::Velocity>(
    Vector2{50.0f, 0.0f}     // velocity
);

// Check for components
if (player.hasComponent<yvium::components::Transform>()) {
    auto* t = player.getComponent<yvium::components::Transform>();
    t->position.x += 10.0f;
}
```

## 📚 Documentation

- **Code Documentation**: All public APIs are documented with Doxygen-style comments
- **Architecture Guide**: See `/docs/architecture.md` (TODO)
- **API Reference**: Generated with Doxygen (TODO)

## 🧪 Testing

The project uses Catch2 for unit and integration testing:

```bash
# Run all tests
./scripts/build.sh --tests

# Run specific test categories
./build/bin/yvium-rlib-tests [entity]
./build/bin/yvium-rlib-tests [component]
```

## 🎯 Roadmap

- [ ] Complete ECS implementation
- [ ] Scene management system
- [ ] Asset loading and management
- [ ] Audio system integration
- [ ] Input handling system
- [ ] Physics integration
- [ ] Rendering pipeline improvements
- [ ] Scripting support (Lua)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Follow the coding style (use clang-format)
4. Add tests for new functionality
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- [raylib](https://www.raylib.com/) - Amazing game development library
- [Catch2](https://github.com/catchorg/Catch2) - Modern C++ testing framework
- The C++ game development community