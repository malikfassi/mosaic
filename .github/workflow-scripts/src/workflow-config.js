// Job name definitions
export const JOBS = {
  // Frontend jobs
  FRONTEND_LINT: 'frontend-ci-lint',
  FRONTEND_TEST: 'frontend-ci-test',
  FRONTEND_BUILD: 'frontend-ci-build',

  // Mosaic Tile jobs
  MOSAIC_TILE_FORMAT: 'mosaic-tile-ci-format',
  MOSAIC_TILE_LINT: 'mosaic-tile-ci-lint',
  MOSAIC_TILE_TEST: 'mosaic-tile-ci-test',
  MOSAIC_TILE_SCHEMA: 'mosaic-tile-ci-schema',
  MOSAIC_TILE_DEPLOY: 'deploy-mosaic-tile',
  MOSAIC_TILE_E2E: 'mosaic-tile-e2e',

  // Mosaic Vending jobs
  MOSAIC_VENDING_FORMAT: 'mosaic-vending-ci-format',
  MOSAIC_VENDING_LINT: 'mosaic-vending-ci-lint',
  MOSAIC_VENDING_TEST: 'mosaic-vending-ci-test',
  MOSAIC_VENDING_SCHEMA: 'mosaic-vending-ci-schema',
  MOSAIC_VENDING_DEPLOY: 'deploy-mosaic-vending',
  MOSAIC_VENDING_E2E: 'mosaic-vending-e2e',

  // Integration jobs
  FULL_E2E: 'full-e2e'
};

// Component definitions with their file paths
export const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
    jobs: [
      JOBS.FRONTEND_LINT,
      JOBS.FRONTEND_TEST,
      JOBS.FRONTEND_BUILD
    ]
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml'],
    jobs: [
      JOBS.MOSAIC_TILE_FORMAT,
      JOBS.MOSAIC_TILE_LINT,
      JOBS.MOSAIC_TILE_TEST,
      JOBS.MOSAIC_TILE_SCHEMA,
      JOBS.MOSAIC_TILE_DEPLOY,
      JOBS.MOSAIC_TILE_E2E
    ]
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml'],
    jobs: [
      JOBS.MOSAIC_VENDING_FORMAT,
      JOBS.MOSAIC_VENDING_LINT,
      JOBS.MOSAIC_VENDING_TEST,
      JOBS.MOSAIC_VENDING_SCHEMA,
      JOBS.MOSAIC_VENDING_DEPLOY,
      JOBS.MOSAIC_VENDING_E2E
    ]
  }
};


export function getAllFileNames(componentHashes, commit_sha) {
  const allJobs = getAllJobs();
  return Object.entries(allJobs).map(([jobName, jobConfig]) => {
    const hash = componentHashes[jobConfig.component];
    if (jobName === JOBS.FULL_E2E) {
      return `${jobName}.${commit_sha}.json`;
    }
    return `${jobName}.${hash}.json`;
  });
}

// Helper to get all jobs for a component
export function getComponentJobs(component) {
  return COMPONENTS[component]?.jobs || [];
}

// Helper to get all jobs including integration jobs
export function getAllJobs() {
  const jobs = {};
  
  // Add component jobs
  Object.entries(COMPONENTS).forEach(([componentName, component]) => {
    component.jobs.forEach(jobName => {
      jobs[jobName] = { component: componentName };
    });
  });
  
  // Add integration jobs
  jobs[JOBS.FULL_E2E] = { component: 'integration' };
  
  return jobs;
} 