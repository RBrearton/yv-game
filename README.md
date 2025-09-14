# Yvium Game

A simple C17 game built with raylib.

## 🚀 Features

- **Modern C17**: Clean, efficient code
- **Cross-platform**: Works on Windows, macOS, and Linux  
- **Modern Build System**: CMake with automatic raylib detection
- **Code Quality**: Integrated linting and formatting

## 📁 Project Structure

```
yvium_rlib/
├── assets/                   # Game assets
│   ├── textures/            # Image files and sprites
│   ├── sounds/              # Audio files
│   ├── fonts/               # Font files
│   ├── shaders/             # Shader programs
│   └── data/                # Game data files
├── build/                   # Build output (auto-generated)
├── cmake/                   # Custom CMake modules
├── scripts/                 # Build and utility scripts
├── src/                     # Source files
│   └── main.c               # Application entry point
└── tools/                   # Development tools
```

## 🛠️ Prerequisites

- **C17 compatible compiler**:
  - GCC 7+ or Clang 6+ (Linux/macOS)
  - Visual Studio 2017+ or MSVC 19.14+ (Windows)
- **CMake 3.20+**
- **Git** (for dependency fetching)

### Optional Tools
- **clang-format** (for code formatting)
- **clangd** (for LSP support)
- **Doxygen** (for documentation generation)

## 🚀 Quick Start

### 1. Clone and Run

```bash
git clone <repository-url> yvium_rlib
cd yvium_rlib

# Build and run (like `cargo run`)
./run

# Or use the full script with options
./scripts/run.sh --release
```

### Development Workflow

```bash
# Quick development cycle (debug build + run)
./run

# Release build and run  
./run --release

# Clean build and run
./run --clean

# Just build (without running)
./scripts/build.sh
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

## 🎮 Simple Architecture

The project uses a straightforward approach:

- **main.c**: Contains the complete game loop
- **raylib**: Handles graphics, input, and audio
- **Your code**: Add your game logic directly to main.c or create additional files as needed

### Example Usage

```c
#include <raylib.h>

int main(void) {
    InitWindow(1280, 720, "My Game");
    SetTargetFPS(60);
    
    while (!WindowShouldClose()) {
        // Your game logic here
        
        BeginDrawing();
        ClearBackground(RAYWHITE);
        DrawText("Hello World!", 190, 200, 20, LIGHTGRAY);
        EndDrawing();
    }
    
    CloseWindow();
    return 0;
}
```

## 📚 Documentation

- **raylib Documentation**: [Official raylib docs](https://www.raylib.com/)
- **C17 Reference**: For modern C features

## 🧪 Testing

Testing framework removed for simplicity. Add your own tests if needed.

## 🎯 Getting Started

1. **Quick start**: `./run` (builds and runs in one command)
2. **Start coding**: Edit `src/main.c` 
3. **Iterate**: `./run` after each change

Just like `cargo run` in Rust! 🦀

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