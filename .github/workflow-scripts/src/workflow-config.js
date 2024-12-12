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
      'contracts/mosaic-tile-nft/**/*',
      'contracts/Cargo.toml',
    ]
  },
  mosaic_vending: {
    name: "mosaic_vending",
    paths: [
      'contracts/mosaic-vending-minter/**/*',
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
  mosaic_tile_clippy: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_fmt: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_test: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_compile: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_deploy: { component: COMPONENTS.mosaic_tile },
  mosaic_tile_e2e: { component: COMPONENTS.mosaic_tile },
  
  // Mosaic Vending jobs
  mosaic_vending_clippy: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_fmt: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_test: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_compile: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_deploy: { component: COMPONENTS.mosaic_vending },
  mosaic_vending_e2e: { component: COMPONENTS.mosaic_vending },

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