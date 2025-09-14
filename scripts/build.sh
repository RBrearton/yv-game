#!/bin/bash

# Build script for yv game project.

set -e  # Exit on any error.

# Colors for output.
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color.

# Default values.
BUILD_TYPE="Release"
BUILD_TESTS=OFF
CLEAN_BUILD=false
JOBS=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)

# Function to print colored output.
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to show usage.
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  -h, --help          Show this help message"
    echo "  -d, --debug         Build in Debug mode (default: Release)"
    echo "  -t, --tests         Build with tests enabled"
    echo "  -c, --clean         Clean build directory before building"
    echo "  -j, --jobs N        Number of parallel jobs (default: auto-detected)"
    echo ""
    echo "Examples:"
    echo "  $0                  # Release build"
    echo "  $0 -d               # Debug build"
    echo "  $0 -t               # Release build with tests"
    echo "  $0 -d -t -c         # Clean debug build with tests"
}

# Parse command line arguments.
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -d|--debug)
            BUILD_TYPE="Debug"
            shift
            ;;
        -t|--tests)
            BUILD_TESTS=ON
            shift
            ;;
        -c|--clean)
            CLEAN_BUILD=true
            shift
            ;;
        -j|--jobs)
            JOBS="$2"
            shift 2
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Get script directory.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BUILD_DIR="$PROJECT_ROOT/build"

print_info "Building yv game project..."
print_info "Build type: $BUILD_TYPE"
print_info "Tests: $BUILD_TESTS"
print_info "Jobs: $JOBS"

# Clean build directory if requested.
if [ "$CLEAN_BUILD" = true ]; then
    print_info "Cleaning build directory..."
    rm -rf "$BUILD_DIR"
fi

# Create build directory.
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# Configure with CMake.
print_info "Configuring with CMake..."
cmake .. \
    -DCMAKE_BUILD_TYPE="$BUILD_TYPE" \
    -DBUILD_TESTS="$BUILD_TESTS" \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON

# Build the project.
print_info "Building project..."
cmake --build . --config "$BUILD_TYPE" -j "$JOBS"

# Run tests if enabled.
if [ "$BUILD_TESTS" = "ON" ]; then
    print_info "Running tests..."
    ctest --output-on-failure
fi

print_info "Build completed successfully!"
print_info "Executable location: $BUILD_DIR/bin/yv-rlib"
