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
  frontend_ci_lint: { component: COMPONENTS.frontend },
  frontend_ci_test: { component: COMPONENTS.frontend },
  frontend_ci_build: { component: COMPONENTS.frontend },
  
  // Mosaic Tile jobs
  clippy_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  fmt_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  test_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  compile_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  deploy_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  e2e_mosaic_tile: { component: COMPONENTS.mosaic_tile },
  
  // Mosaic Vending jobs
  clippy_mosaic_vending: { component: COMPONENTS.mosaic_vending },
  fmt_mosaic_vending: { component: COMPONENTS.mosaic_vending },
  test_mosaic_vending: { component: COMPONENTS.mosaic_vending },
  compile_mosaic_vending: { component: COMPONENTS.mosaic_vending },
  deploy_mosaic_vending: { component: COMPONENTS.mosaic_vending },
  e2e_mosaic_vending: { component: COMPONENTS.mosaic_vending },

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