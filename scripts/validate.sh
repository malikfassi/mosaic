#!/bin/bash

# Exit on error
set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

# Cache directory for validation state
CACHE_DIR=".validate-cache"
FRONTEND_HASH_FILE="$CACHE_DIR/frontend-hash"
CONTRACT_HASH_FILE="$CACHE_DIR/contract-hash"
mkdir -p "$CACHE_DIR"

# Function to print section header
print_header() {
    echo -e "\n${BLUE}=== $1 ===${NC}"
}

# Function to handle errors
handle_error() {
    echo -e "\n${RED}Error in $1: $2${NC}"
    return 1
}

# Function to calculate hash of directory
calculate_dir_hash() {
    if [ -d "$1" ]; then
        find "$1" -type f \
            -not -path "*/node_modules/*" \
            -not -path "*/target/*" \
            -not -path "*/.next/*" \
            -not -path "*/coverage/*" \
            -exec sha256sum {} \; | sort | sha256sum | cut -d' ' -f1
    else
        echo ""
    fi
}

# Function to check if validation is needed
needs_validation() {
    local dir=$1
    local hash_file=$2
    
    # If hash file doesn't exist, validation is needed
    if [ ! -f "$hash_file" ]; then
        return 0
    fi
    
    local current_hash=$(calculate_dir_hash "$dir")
    local stored_hash=$(cat "$hash_file")
    
    # If hashes are different, validation is needed
    [ "$current_hash" != "$stored_hash" ]
}

# Function to update hash after successful validation
update_hash() {
    local dir=$1
    local hash_file=$2
    calculate_dir_hash "$dir" > "$hash_file"
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

# If no specific changes, check all directories
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
    # Clean install
    rm -rf node_modules .next coverage
    if ! npm install; then
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

# Run validations in parallel
FRONTEND_STATUS=0
CONTRACT_STATUS=0

if [ "$RUN_FRONTEND" = true ]; then
    if needs_validation "frontend" "$FRONTEND_HASH_FILE"; then
        run_frontend_validation &
        FRONTEND_PID=$!
    else
        echo -e "${GREEN}Frontend validation skipped (no changes detected)${NC}"
    fi
fi

if [ "$RUN_CONTRACT" = true ]; then
    if needs_validation "contracts" "$CONTRACT_HASH_FILE"; then
        run_contract_validation &
        CONTRACT_PID=$!
    else
        echo -e "${GREEN}Contract validation skipped (no changes detected)${NC}"
    fi
fi

# Wait for all background processes to complete
if [ "$RUN_FRONTEND" = true ] && needs_validation "frontend" "$FRONTEND_HASH_FILE"; then
    wait $FRONTEND_PID || FRONTEND_STATUS=$?
    if [ $FRONTEND_STATUS -eq 0 ]; then
        update_hash "frontend" "$FRONTEND_HASH_FILE"
    fi
fi

if [ "$RUN_CONTRACT" = true ] && needs_validation "contracts" "$CONTRACT_HASH_FILE"; then
    wait $CONTRACT_PID || CONTRACT_STATUS=$?
    if [ $CONTRACT_STATUS -eq 0 ]; then
        update_hash "contracts" "$CONTRACT_HASH_FILE"
    fi
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