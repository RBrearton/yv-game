#!/bin/bash

# Code formatting script for Yvium project.

set -e  # Exit on any error.

# Colors for output.
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color.

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
    echo "  -c, --check         Check formatting without modifying files"
    echo "  -f, --fix           Fix formatting issues (default behavior)"
    echo ""
    echo "Examples:"
    echo "  $0                  # Format all files"
    echo "  $0 --check          # Check formatting without changes"
}

# Default values.
CHECK_ONLY=false

# Parse command line arguments.
while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -c|--check)
            CHECK_ONLY=true
            shift
            ;;
        -f|--fix)
            CHECK_ONLY=false
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Get script directory and project root.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Check if clang-format is available.
if ! command -v clang-format &> /dev/null; then
    print_error "clang-format is not installed!"
    print_info "Install it with: brew install clang-format"
    exit 1
fi

print_info "Using clang-format version: $(clang-format --version | cut -d' ' -f3)"

# Find all C++ source and header files.
CPP_FILES=$(find "$PROJECT_ROOT/src" "$PROJECT_ROOT/include" \
    -name "*.cpp" -o -name "*.hpp" -o -name "*.c" -o -name "*.h" \
    2>/dev/null || true)

if [ -z "$CPP_FILES" ]; then
    print_warning "No C++ files found to format."
    exit 0
fi

FILE_COUNT=$(echo "$CPP_FILES" | wc -l | tr -d ' ')
print_info "Found $FILE_COUNT C++ files to process."

if [ "$CHECK_ONLY" = true ]; then
    print_info "Checking code formatting..."
    
    # Check each file and collect results.
    ISSUES_FOUND=false
    
    for file in $CPP_FILES; do
        if ! clang-format --dry-run --Werror "$file" &>/dev/null; then
            if [ "$ISSUES_FOUND" = false ]; then
                print_warning "Formatting issues found in:"
                ISSUES_FOUND=true
            fi
            echo "  - $(basename "$file")"
        fi
    done
    
    if [ "$ISSUES_FOUND" = true ]; then
        print_error "Code formatting issues found!"
        print_info "Run '$0 --fix' to automatically fix them."
        exit 1
    else
        print_info "All files are properly formatted!"
        exit 0
    fi
else
    print_info "Formatting code..."
    
    # Format all files.
    echo "$CPP_FILES" | xargs clang-format -i
    
    print_info "Code formatting completed!"
    print_info "All $FILE_COUNT files have been formatted according to .clang-format rules."
fi
