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

# Function to handle errors
handle_error() {
    echo -e "\n${RED}Error in $1: $2${NC}"
    return 1
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

# Frontend validation function
run_frontend_validation() {
    print_header "Frontend Validation"
    cd frontend || return 1

    echo -e "${YELLOW}Installing dependencies...${NC}"
    if ! npm ci; then
        handle_error "Frontend" "Failed to install dependencies"
        return 1
    fi

    echo -e "\n${YELLOW}Type checking...${NC}"
    if ! npm run type-check; then
        handle_error "Frontend" "Type check failed"
        return 1
    fi

    echo -e "\n${YELLOW}Linting...${NC}"
    if ! npm run lint; then
        handle_error "Frontend" "Linting failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running tests...${NC}"
    if ! npm run test; then
        handle_error "Frontend" "Tests failed"
        return 1
    fi

    echo -e "\n${YELLOW}Building...${NC}"
    if ! npm run build; then
        handle_error "Frontend" "Build failed"
        return 1
    fi

    cd ..
    echo -e "${GREEN}Frontend validation completed successfully${NC}"
    return 0
}

# Contract validation function
run_contract_validation() {
    print_header "Contract Validation"
    cd contracts/pixel-canvas || return 1

    echo -e "${YELLOW}Checking...${NC}"
    if ! cargo check; then
        handle_error "Contract" "Cargo check failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running clippy...${NC}"
    if ! cargo clippy -- -D warnings; then
        handle_error "Contract" "Clippy check failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running tests...${NC}"
    if ! cargo test; then
        handle_error "Contract" "Tests failed"
        return 1
    fi

    echo -e "\n${YELLOW}Building...${NC}"
    if ! cargo build; then
        handle_error "Contract" "Build failed"
        return 1
    fi

    cd ../../
    echo -e "${GREEN}Contract validation completed successfully${NC}"
    return 0
}

# Run validations in parallel and capture their output
FRONTEND_OUTPUT=""
CONTRACT_OUTPUT=""
FRONTEND_STATUS=0
CONTRACT_STATUS=0

if [ "$RUN_FRONTEND" = true ]; then
    # Run frontend validation in background and redirect output to a file descriptor
    exec 3>&1
    FRONTEND_OUTPUT=$(run_frontend_validation 2>&1 | tee /dev/fd/3; exit ${PIPESTATUS[0]}) &
    FRONTEND_PID=$!
fi

if [ "$RUN_CONTRACT" = true ]; then
    # Run contract validation in background and redirect output to a file descriptor
    exec 4>&1
    CONTRACT_OUTPUT=$(run_contract_validation 2>&1 | tee /dev/fd/4; exit ${PIPESTATUS[0]}) &
    CONTRACT_PID=$!
fi

# Wait for all background processes to complete
if [ "$RUN_FRONTEND" = true ]; then
    wait $FRONTEND_PID || FRONTEND_STATUS=$?
fi

if [ "$RUN_CONTRACT" = true ]; then
    wait $CONTRACT_PID || CONTRACT_STATUS=$?
fi

# Check for errors
FAILED=false

if [ $FRONTEND_STATUS -ne 0 ]; then
    FAILED=true
fi

if [ $CONTRACT_STATUS -ne 0 ]; then
    FAILED=true
fi

if [ "$FAILED" = true ]; then
    print_header "Validation Failed"
    exit 1
else
    print_header "Validation Complete"
    echo -e "${GREEN}All validation checks passed!${NC}"
fi

# Optional: Check for uncommitted changes
if [ -n "$(git status --porcelain)" ]; then
    echo -e "\n${YELLOW}Warning: You have uncommitted changes${NC}"
    git status --short
fi 