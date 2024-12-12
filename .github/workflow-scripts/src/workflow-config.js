// Define components and their file paths for hashing
export const COMPONENTS = {
  frontend: {
    name: "frontend",
    paths: [
      'frontend/**/*',
    ]
  },
  mosaic_tile: {
    name: "mosaic_tile",
    paths: [
      'contracts/mosaic_tile_nft/**/*',
      'contracts/Cargo.toml',
    ]
  },
  mosaic_vending: {
    name: "mosaic_vending",
    paths: [
      'contracts/mosaic_vending_minter/**/*',
      'contracts/Cargo.toml',
    ]
  },
  all: {
    name: "all",
    paths: [
        "."
    ]
  }
};

// Explicit list of all jobs and their components
export const JOBS = {
  // Frontend jobs
  frontend_lint: { component: COMPONENTS.frontend },
  frontend_test: { component: COMPONENTS.frontend },
  frontend_build: { component: COMPONENTS.frontend },
  
  // Mosaic Tile jobs
  mosaic_tile_nft_clippy: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_nft_fmt: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_nft_test: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_nft_compile: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_nft_deploy: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_nft_e2e: { component: COMPONENTS.mosaic_tile },
  
  // Mosaic Vending jobs
  mosaic_vending_minter_clippy: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_minter_fmt: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_minter_test: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_minter_compile: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_minter_deploy: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_minter_e2e: { component: COMPONENTS.mosaic_vending },

  // Full e2e jobs
  full_e2e: { component: COMPONENTS.all }
};

export function tryParseJson(str) {
  if (!str) return null;
  try {
    return JSON.parse(str);
  } catch (error) {
    console.warn('Failed to parse JSON:', error.message);
    return null;
  }
}