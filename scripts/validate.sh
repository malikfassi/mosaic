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
NFT_HASH_FILE="$CACHE_DIR/nft-hash"
COLORING_HASH_FILE="$CACHE_DIR/coloring-hash"
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

# Check if we should run validations based on changed files
CHANGED_FILES=$(git diff --cached --name-only)
RUN_FRONTEND=false
RUN_NFT=false
RUN_COLORING=false

if [ -z "$CHANGED_FILES" ]; then
    # No staged files, check working directory changes
    CHANGED_FILES=$(git status --porcelain | awk '{print $2}')
fi

if echo "$CHANGED_FILES" | grep -q "^frontend/"; then
    RUN_FRONTEND=true
fi

if echo "$CHANGED_FILES" | grep -q "^contracts/sg721-pixel/"; then
    RUN_NFT=true
fi

if echo "$CHANGED_FILES" | grep -q "^contracts/pixel-coloring/"; then
    RUN_COLORING=true
fi

# If no specific changes, check all directories
if [ "$RUN_FRONTEND" = false ] && [ "$RUN_NFT" = false ] && [ "$RUN_COLORING" = false ]; then
    RUN_FRONTEND=true
    RUN_NFT=true
    RUN_COLORING=true
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

    echo -e "\n${YELLOW}Running static validation...${NC}"
    if ! npm run lint; then
        handle_error "Frontend" "Linting failed"
        return 1
    fi

    if ! npm run typecheck; then
        handle_error "Frontend" "Type checking failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running tests...${NC}"
    if ! npm test; then
        handle_error "Frontend" "Tests failed"
        return 1
    fi

    cd ..
    echo -e "${GREEN}Frontend validation completed successfully${NC}"
    return 0
}

# NFT contract validation function
run_nft_validation() {
    print_header "NFT Contract Validation"
    cd contracts/sg721-pixel || return 1

    echo -e "${YELLOW}Running static validation...${NC}"
    if ! cargo fmt -- --check; then
        handle_error "NFT Contract" "Format check failed"
        return 1
    fi

    if ! cargo clippy --all-targets --all-features; then
        handle_error "NFT Contract" "Clippy check failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running tests...${NC}"
    if ! cargo test --verbose --all-features; then
        handle_error "NFT Contract" "Tests failed"
        return 1
    fi

    cd ../../
    echo -e "${GREEN}NFT contract validation completed successfully${NC}"
    return 0
}

# Coloring contract validation function
run_coloring_validation() {
    print_header "Coloring Contract Validation"
    cd contracts/pixel-coloring || return 1

    echo -e "${YELLOW}Running static validation...${NC}"
    if ! cargo fmt -- --check; then
        handle_error "Coloring Contract" "Format check failed"
        return 1
    fi

    if ! cargo clippy --all-targets --all-features; then
        handle_error "Coloring Contract" "Clippy check failed"
        return 1
    fi

    echo -e "\n${YELLOW}Running tests...${NC}"
    if ! cargo test --verbose --all-features; then
        handle_error "Coloring Contract" "Tests failed"
        return 1
    fi

    cd ../../
    echo -e "${GREEN}Coloring contract validation completed successfully${NC}"
    return 0
}

# Run validations in parallel
FRONTEND_STATUS=0
NFT_STATUS=0
COLORING_STATUS=0

if [ "$RUN_FRONTEND" = true ]; then
    if needs_validation "frontend" "$FRONTEND_HASH_FILE"; then
        run_frontend_validation &
        FRONTEND_PID=$!
    else
        echo -e "${GREEN}Frontend validation skipped (no changes detected)${NC}"
    fi
fi

if [ "$RUN_NFT" = true ]; then
    if needs_validation "contracts/sg721-pixel" "$NFT_HASH_FILE"; then
        run_nft_validation &
        NFT_PID=$!
    else
        echo -e "${GREEN}NFT contract validation skipped (no changes detected)${NC}"
    fi
fi

if [ "$RUN_COLORING" = true ]; then
    if needs_validation "contracts/pixel-coloring" "$COLORING_HASH_FILE"; then
        run_coloring_validation &
        COLORING_PID=$!
    else
        echo -e "${GREEN}Coloring contract validation skipped (no changes detected)${NC}"
    fi
fi

# Wait for all background processes to complete
if [ "$RUN_FRONTEND" = true ] && needs_validation "frontend" "$FRONTEND_HASH_FILE"; then
    wait $FRONTEND_PID || FRONTEND_STATUS=$?
    if [ $FRONTEND_STATUS -eq 0 ]; then
        update_hash "frontend" "$FRONTEND_HASH_FILE"
    fi
fi

if [ "$RUN_NFT" = true ] && needs_validation "contracts/sg721-pixel" "$NFT_HASH_FILE"; then
    wait $NFT_PID || NFT_STATUS=$?
    if [ $NFT_STATUS -eq 0 ]; then
        update_hash "contracts/sg721-pixel" "$NFT_HASH_FILE"
    fi
fi

if [ "$RUN_COLORING" = true ] && needs_validation "contracts/pixel-coloring" "$COLORING_HASH_FILE"; then
    wait $COLORING_PID || COLORING_STATUS=$?
    if [ $COLORING_STATUS -eq 0 ]; then
        update_hash "contracts/pixel-coloring" "$COLORING_HASH_FILE"
    fi
fi

# Check for errors
FAILED=false

if [ $FRONTEND_STATUS -ne 0 ]; then
    FAILED=true
fi

if [ $NFT_STATUS -ne 0 ]; then
    FAILED=true
fi

if [ $COLORING_STATUS -ne 0 ]; then
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