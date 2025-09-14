#!/bin/bash

# Quick build-and-run script for development (like `cargo run`).

set -e  # Exit on any error.

# Colors for output.
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color.

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Get script directory and project root.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/build"

# Default values.
BUILD_TYPE="Debug"
CLEAN_BUILD=false

# Parse command line arguments.
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Quick build-and-run script (like 'cargo run')"
            echo ""
            echo "Options:"
            echo "  -h, --help          Show this help message"
            echo "  -r, --release       Build and run in release mode"
            echo "  -c, --clean         Clean build before running"
            echo ""
            echo "Examples:"
            echo "  $0                  # Debug build and run"
            echo "  $0 --release        # Release build and run"
            echo "  $0 --clean          # Clean debug build and run"
            exit 0
            ;;
        -r|--release)
            BUILD_TYPE="Release"
            shift
            ;;
        -c|--clean)
            CLEAN_BUILD=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            echo "Use --help for usage information."
            exit 1
            ;;
    esac
done

print_info "Building and running in $BUILD_TYPE mode..."

# Clean if requested.
if [ "$CLEAN_BUILD" = true ]; then
    print_info "Cleaning build directory..."
    rm -rf "$BUILD_DIR"
fi

# Create build directory if it doesn't exist.
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configure if needed (or if clean build).
if [ ! -f "Makefile" ] || [ "$CLEAN_BUILD" = true ]; then
    print_info "Configuring with CMake..."
    cmake .. -DCMAKE_BUILD_TYPE="$BUILD_TYPE" -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
fi

# Build the project.
print_info "Building..."
make -j$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

# Run the executable.
EXECUTABLE="$BUILD_DIR/bin/yvium-rlib"
if [ -f "$EXECUTABLE" ]; then
    print_info "Running $EXECUTABLE..."
    echo ""
    "$EXECUTABLE"
else
    print_error "Executable not found: $EXECUTABLE"
    exit 1
fi
