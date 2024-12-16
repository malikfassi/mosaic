import { validateJob } from './utils/jobs.js';

// Component types
const COMPONENT_TYPES = {
    FRONTEND: 'frontend',
    MOSAIC_TILE: 'mosaic_tile',
    MOSAIC_VENDING: 'mosaic_vending',
    ALL: 'all'
};

// Define components and their file paths for hashing
export const COMPONENTS = {
    [COMPONENT_TYPES.FRONTEND]: {
        name: COMPONENT_TYPES.FRONTEND,
        paths: [
            'frontend/**/*',
        ]
    },
    [COMPONENT_TYPES.MOSAIC_TILE]: {
        name: COMPONENT_TYPES.MOSAIC_TILE,
        paths: [
            'contracts/mosaic_tile_nft/**/*',
            'contracts/Cargo.toml',
        ]
    },
    [COMPONENT_TYPES.MOSAIC_VENDING]: {
        name: COMPONENT_TYPES.MOSAIC_VENDING,
        paths: [
            'contracts/mosaic_vending_minter/**/*',
            'contracts/Cargo.toml',
        ]
    },
    [COMPONENT_TYPES.ALL]: {
        name: COMPONENT_TYPES.ALL,
        paths: [
            "."
        ]
    }
};

// Job types
const JOB_TYPES = {
    // Frontend jobs
    FRONTEND_LINT: 'frontend_lint',
    FRONTEND_TEST: 'frontend_test',
    FRONTEND_BUILD: 'frontend_build',
    
    // Mosaic Tile jobs
    MOSAIC_TILE_CLIPPY: 'mosaic_tile_nft_clippy',
    MOSAIC_TILE_FMT: 'mosaic_tile_nft_fmt',
    MOSAIC_TILE_TEST: 'mosaic_tile_nft_test',
    MOSAIC_TILE_COMPILE: 'mosaic_tile_nft_compile',
    MOSAIC_TILE_DEPLOY: 'mosaic_tile_nft_deploy',
    MOSAIC_TILE_E2E: 'mosaic_tile_nft_e2e',
    
    // Mosaic Vending jobs
    MOSAIC_VENDING_CLIPPY: 'mosaic_vending_minter_clippy',
    MOSAIC_VENDING_FMT: 'mosaic_vending_minter_fmt',
    MOSAIC_VENDING_TEST: 'mosaic_vending_minter_test',
    MOSAIC_VENDING_COMPILE: 'mosaic_vending_minter_compile',
    MOSAIC_VENDING_DEPLOY: 'mosaic_vending_minter_deploy',
    MOSAIC_VENDING_E2E: 'mosaic_vending_minter_e2e',

    // Full e2e jobs
    FULL_E2E: 'full_e2e'
};

// Create and validate jobs
const createJobs = () => {
    const jobs = {
        // Frontend jobs
        [JOB_TYPES.FRONTEND_LINT]: { component: COMPONENTS[COMPONENT_TYPES.FRONTEND] },
        [JOB_TYPES.FRONTEND_TEST]: { component: COMPONENTS[COMPONENT_TYPES.FRONTEND] },
        [JOB_TYPES.FRONTEND_BUILD]: { component: COMPONENTS[COMPONENT_TYPES.FRONTEND] },
        
        // Mosaic Tile jobs
        [JOB_TYPES.MOSAIC_TILE_CLIPPY]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        [JOB_TYPES.MOSAIC_TILE_FMT]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        [JOB_TYPES.MOSAIC_TILE_TEST]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        [JOB_TYPES.MOSAIC_TILE_COMPILE]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        [JOB_TYPES.MOSAIC_TILE_DEPLOY]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        [JOB_TYPES.MOSAIC_TILE_E2E]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE] },
        
        // Mosaic Vending jobs
        [JOB_TYPES.MOSAIC_VENDING_CLIPPY]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },
        [JOB_TYPES.MOSAIC_VENDING_FMT]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },
        [JOB_TYPES.MOSAIC_VENDING_TEST]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },
        [JOB_TYPES.MOSAIC_VENDING_COMPILE]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },
        [JOB_TYPES.MOSAIC_VENDING_DEPLOY]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },
        [JOB_TYPES.MOSAIC_VENDING_E2E]: { component: COMPONENTS[COMPONENT_TYPES.MOSAIC_VENDING] },

        // Full e2e jobs
        [JOB_TYPES.FULL_E2E]: { component: COMPONENTS[COMPONENT_TYPES.ALL] }
    };

    // Validate all jobs
    Object.entries(jobs).forEach(([name, job]) => validateJob(job));
    return jobs;
};

// Export validated jobs
export const JOBS = createJobs();

// Export job and component types for use in other modules
export { JOB_TYPES, COMPONENT_TYPES };

// JSON parsing utility
export function tryParseJson(str) {
    if (!str) return null;
    try {
        return JSON.parse(str);
    } catch (error) {
        console.warn('Failed to parse JSON:', error.message);
        return null;
    }
}