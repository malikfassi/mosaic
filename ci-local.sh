#!/bin/bash

set -e  # Exit on any error

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default flags - all disabled by default
RUN_FRONTEND=0
RUN_MOSAIC_TILE=0
RUN_VENDING_MINTER=0
RUN_ADDITIONAL=0
RUN_ALL=0
AUTO_FIX=0

# Parse command line arguments
print_usage() {
    echo "Usage: $0 [options]"
    echo "Options:"
    echo "  -a, --all             Run all tests"
    echo "  -f, --frontend        Run frontend tests"
    echo "  -m, --mosaic-tile     Run mosaic tile contract tests"
    echo "  -v, --vending-minter  Run vending minter contract tests"
    echo "  -x, --extra           Run additional checks (coverage, wasm build)"
    echo "      --fix             Auto-fix linting and formatting issues"
    echo "  -h, --help            Show this help message"
}

while [[ $# -gt 0 ]]; do
    case $1 in
        -a|--all)
            RUN_ALL=1
            shift
            ;;
        -f|--frontend)
            RUN_FRONTEND=1
            shift
            ;;
        -m|--mosaic-tile)
            RUN_MOSAIC_TILE=1
            shift
            ;;
        -v|--vending-minter)
            RUN_VENDING_MINTER=1
            shift
            ;;
        -x|--extra)
            RUN_ADDITIONAL=1
            shift
            ;;
        --fix)
            AUTO_FIX=1
            shift
            ;;
        -h|--help)
            print_usage
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            print_usage
            exit 1
            ;;
    esac
done

# If no flags are specified, show usage
if [[ $RUN_ALL -eq 0 && $RUN_FRONTEND -eq 0 && $RUN_MOSAIC_TILE -eq 0 && \
      $RUN_VENDING_MINTER -eq 0 && $RUN_ADDITIONAL -eq 0 ]]; then
    print_usage
    exit 1
fi

# If --all is specified, enable all flags
if [[ $RUN_ALL -eq 1 ]]; then
    RUN_FRONTEND=1
    RUN_MOSAIC_TILE=1
    RUN_VENDING_MINTER=1
    RUN_ADDITIONAL=1
fi

# Store the root directory
ROOT_DIR=$(pwd)
echo -e "${YELLOW}Starting CI checks from: ${ROOT_DIR}${NC}"

# Validate we're in the project root
echo -e "${BLUE}Validating project structure...${NC}"
MISSING_FILES=()
[ ! -d "contracts" ] && MISSING_FILES+=("contracts/")
[ ! -d "frontend" ] && MISSING_FILES+=("frontend/")
[ ! -f "TODO.md" ] && MISSING_FILES+=("TODO.md")

if [ ${#MISSING_FILES[@]} -ne 0 ]; then
    echo -e "${RED}Error: Must run this script from the project root directory${NC}"
    echo -e "${BLUE}Current directory: ${ROOT_DIR}${NC}"
    echo -e "${RED}Missing required files/directories:${NC}"
    printf "${RED}  - %s${NC}\n" "${MISSING_FILES[@]}"
    exit 1
fi
echo -e "${GREEN}Project structure validated${NC}"

run_step() {
    local step_name=$1
    local dir=$2
    local command=$3
    
    echo -e "\n${YELLOW}=====================================${NC}"
    echo -e "${BLUE}Running step: ${step_name}${NC}"
    echo -e "${BLUE}Directory: ${ROOT_DIR}/${dir}${NC}"
    echo -e "${BLUE}Command: ${command}${NC}"
    echo -e "${YELLOW}=====================================${NC}\n"
    
    if [ ! -d "$ROOT_DIR/$dir" ]; then
        echo -e "\n${RED}✗ Directory not found: $ROOT_DIR/$dir${NC}"
        return 1
    fi
    
    pushd "$ROOT_DIR/$dir" > /dev/null || return 1
    if eval $command; then
        echo -e "\n${GREEN}✓ Step passed: ${step_name}${NC}"
        popd > /dev/null || return 1
        return 0
    else
        echo -e "\n${RED}✗ Step failed: ${step_name}${NC}"
        popd > /dev/null || return 1
        return 1
    fi
}

echo -e "\n${YELLOW}Starting CI checks...${NC}\n"

# Shared Rust Setup (always run if any Rust tests are needed)
if [[ $RUN_MOSAIC_TILE -eq 1 || $RUN_VENDING_MINTER -eq 1 ]]; then
    run_step "Install Rust components" "." "rustup component add rustfmt clippy" || exit 1
    [[ $RUN_ADDITIONAL -eq 1 ]] && run_step "Install cargo-tarpaulin" "." "cargo install cargo-tarpaulin" || true
fi

# Frontend Tests
if [[ $RUN_FRONTEND -eq 1 ]]; then
    echo -e "\n${YELLOW}Running Frontend Tests${NC}"
    run_step "Frontend install" "frontend" "npm ci" || exit 1
    if [[ $AUTO_FIX -eq 1 ]]; then
        run_step "Frontend lint" "frontend" "npm run lint -- --fix" || exit 1
    else
        run_step "Frontend lint" "frontend" "npm run lint" || exit 1
    fi
    run_step "Frontend tests" "frontend" "npm test" || exit 1
fi

# Mosaic Tile NFT Contract Tests
if [[ $RUN_MOSAIC_TILE -eq 1 ]]; then
    echo -e "\n${YELLOW}Running Mosaic Tile Tests${NC}"
    if [[ $AUTO_FIX -eq 1 ]]; then
        run_step "Mosaic Tile format" "contracts/mosaic-tile-nft" "cargo fmt" || exit 1
        run_step "Mosaic Tile clippy" "contracts/mosaic-tile-nft" "cargo clippy --fix -- -D warnings" || exit 1
    else
        run_step "Mosaic Tile format" "contracts/mosaic-tile-nft" "cargo fmt -- --check" || exit 1
        run_step "Mosaic Tile clippy" "contracts/mosaic-tile-nft" "cargo clippy -- -D warnings" || exit 1
    fi
    run_step "Mosaic Tile tests" "contracts/mosaic-tile-nft" "cargo test" || exit 1
    run_step "Mosaic Tile schema" "contracts/mosaic-tile-nft" "cargo schema" || exit 1
fi

# Mosaic Vending Minter Contract Tests
if [[ $RUN_VENDING_MINTER -eq 1 ]]; then
    echo -e "\n${YELLOW}Running Vending Minter Tests${NC}"
    if [[ $AUTO_FIX -eq 1 ]]; then
        run_step "Vending Minter format" "contracts/mosaic-vending-minter" "cargo fmt" || exit 1
        run_step "Vending Minter clippy" "contracts/mosaic-vending-minter" "cargo clippy --fix -- -D warnings" || exit 1
    else
        run_step "Vending Minter format" "contracts/mosaic-vending-minter" "cargo fmt -- --check" || exit 1
        run_step "Vending Minter clippy" "contracts/mosaic-vending-minter" "cargo clippy -- -D warnings" || exit 1
    fi
    run_step "Vending Minter tests" "contracts/mosaic-vending-minter" "cargo test" || exit 1
    run_step "Vending Minter schema" "contracts/mosaic-vending-minter" "cargo schema" || exit 1
fi

# Additional Contract Checks
if [[ $RUN_ADDITIONAL -eq 1 ]]; then
    echo -e "\n${YELLOW}Running Additional Checks${NC}"
    run_step "Coverage" "." "cargo tarpaulin --verbose --workspace --timeout 120" || exit 1
    run_step "WASM build" "." "cargo build --release --target wasm32-unknown-unknown" || exit 1
    
    # Optional: Only if cargo-deny is installed
    if command -v cargo-deny &> /dev/null; then
        run_step "Cargo deny check" "." "cargo deny check --workspace" || exit 1
    fi
fi

echo -e "\n${GREEN}All requested CI checks completed successfully!${NC}"

# Current CI seems basic
# Should enhance with:
# 1. Multiple rust toolchain testing
# 2. Contract optimization checks
# 3. Gas estimation tests
# 4. Security scanning