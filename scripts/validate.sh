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

# Start validation
print_header "Starting Validation Checks"

# Frontend validation
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

# Contract validation
print_header "Contract Validation"
cd ../contracts/pixel-canvas || handle_error "Contract directory not found"

print_step "1. Cargo check"
cargo check || handle_error "Cargo check failed"

print_step "2. Clippy (no warnings)"
cargo clippy -- -D warnings || handle_error "Clippy check failed"

print_step "3. Tests with coverage"
cargo test || handle_error "Tests failed"

print_step "4. Build"
cargo build || handle_error "Build failed"

# Final check
print_header "Validation Complete"
echo -e "${GREEN}All validation checks passed!${NC}"

# Return to root directory
cd ../../

# Optional: Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo -e "\n${YELLOW}Warning: You have uncommitted changes${NC}"
    git status --short
fi 