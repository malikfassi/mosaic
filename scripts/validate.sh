#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Function to print section header
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Function to print step
print_step() {
    echo -e "\n${YELLOW}$1${NC}"
}

# Function to handle errors
handle_error() {
    echo -e "\n${RED}Error: $1${NC}"
    exit 1
}

# Check if we should run frontend or contract validation based on changed files
CHANGED_FILES=$(git diff --cached --name-only)
RUN_FRONTEND=false
RUN_CONTRACT=false

if [ -z "$CHANGED_FILES" ]; then
    # No staged files, check working directory changes
    CHANGED_FILES=$(git status --porcelain | awk '{print $2}')
fi

if echo "$CHANGED_FILES" | grep -q "^frontend/"; then
    RUN_FRONTEND=true
fi

if echo "$CHANGED_FILES" | grep -q "^contracts/"; then
    RUN_CONTRACT=true
fi

# If no specific changes, run everything
if [ "$RUN_FRONTEND" = false ] && [ "$RUN_CONTRACT" = false ]; then
    RUN_FRONTEND=true
    RUN_CONTRACT=true
fi

# Start validation
print_header "Starting Validation Checks"

# Frontend validation
if [ "$RUN_FRONTEND" = true ]; then
    print_header "Frontend Validation"
    cd frontend || handle_error "Frontend directory not found"

    print_step "1. Checking dependencies"
    npm ci || handle_error "Failed to install dependencies"

    print_step "2. Type checking"
    npm run type-check || handle_error "Type check failed"

    print_step "3. Linting"
    npm run lint || handle_error "Linting failed"

    print_step "4. Tests with coverage"
    npm run test || handle_error "Tests failed"

    # Check test coverage
    if ! npm run test | grep -q "All files.*\|.*100"; then
        handle_error "Test coverage below requirements"
    fi

    print_step "5. Build"
    npm run build || handle_error "Build failed"

    cd .. || handle_error "Failed to return to root"
fi

# Contract validation
if [ "$RUN_CONTRACT" = true ]; then
    print_header "Contract Validation"
    cd contracts/pixel-canvas || handle_error "Contract directory not found"

    print_step "1. Cargo check"
    cargo check || handle_error "Cargo check failed"

    print_step "2. Clippy (no warnings)"
    cargo clippy -- -D warnings || handle_error "Clippy check failed"

    print_step "3. Tests with coverage"
    cargo test || handle_error "Tests failed"

    print_step "4. Build"
    cargo build || handle_error "Build failed"

    cd ../../ || handle_error "Failed to return to root"
fi

# Final check
print_header "Validation Complete"
echo -e "${GREEN}All validation checks passed!${NC}"

# Optional: Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo -e "\n${YELLOW}Warning: You have uncommitted changes${NC}"
    git status --short
fi 