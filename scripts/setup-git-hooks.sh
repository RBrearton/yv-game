#!/bin/bash

# Setup Git hooks for the Yvium project.

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
HOOKS_DIR="$PROJECT_ROOT/.git/hooks"

# Check if we're in a git repository.
if [ ! -d "$PROJECT_ROOT/.git" ]; then
    print_error "Not in a git repository!"
    print_info "Run 'git init' first to initialize the repository."
    exit 1
fi

print_info "Setting up Git hooks for code quality..."

# Create pre-commit hook for formatting.
cat > "$HOOKS_DIR/pre-commit" << 'EOF'
#!/bin/bash

# Pre-commit hook that checks code formatting.

# Check if clang-format is available.
if ! command -v clang-format &> /dev/null; then
    echo "Error: clang-format is not installed!"
    echo "Install it with: brew install clang-format"
    exit 1
fi

# Get the project root.
PROJECT_ROOT=$(git rev-parse --show-toplevel)

# Find staged C files.
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM | grep -E '\.(c|h)$' || true)

if [ -z "$STAGED_FILES" ]; then
    # No C files staged, allow commit.
    exit 0
fi

echo "Checking code formatting..."

# Check each staged file for formatting issues.
FORMATTING_ISSUES=false

for file in $STAGED_FILES; do
    if [ -f "$PROJECT_ROOT/$file" ]; then
        if ! clang-format --dry-run --Werror "$PROJECT_ROOT/$file" &>/dev/null; then
            if [ "$FORMATTING_ISSUES" = false ]; then
                echo "Code formatting issues found in staged files:"
                FORMATTING_ISSUES=true
            fi
            echo "  - $file"
        fi
    fi
done

if [ "$FORMATTING_ISSUES" = true ]; then
    echo ""
    echo "❌ Commit blocked: Code formatting issues detected!"
    echo ""
    echo "Fix formatting with:"
    echo "  ./scripts/format.sh --fix"
    echo ""
    echo "Or check specific issues with:"
    echo "  ./scripts/format.sh --check"
    echo ""
    echo "Then stage your changes and commit again."
    exit 1
fi

echo "✅ Code formatting check passed!"
exit 0
EOF

# Make the hook executable.
chmod +x "$HOOKS_DIR/pre-commit"

print_info "Pre-commit hook installed successfully!"
print_info ""
print_info "The hook will:"
print_info "  ✅ Check code formatting before each commit"
print_info "  ❌ Block commits with formatting issues"
print_info "  💡 Provide instructions to fix issues"
print_info ""
print_info "To bypass the hook (not recommended):"
print_info "  git commit --no-verify"
print_info ""
print_info "To check formatting manually:"
print_info "  ./scripts/format.sh --check"
print_info ""
print_info "To fix formatting:"
print_info "  ./scripts/format.sh --fix"
