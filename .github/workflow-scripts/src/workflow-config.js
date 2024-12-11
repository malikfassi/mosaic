// Component definitions with their file paths
export const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
    jobs: {
      'frontend-ci-lint': { type: 'ci' },
      'frontend-ci-test': { type: 'ci', needs: ['frontend-ci-lint'] },
      'frontend-ci-build': { type: 'ci', needs: ['frontend-ci-lint', 'frontend-ci-test'] }
    }
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml'],
    jobs: {
      'mosaic-tile-ci-format': { type: 'ci' },
      'mosaic-tile-ci-lint': { type: 'ci' },
      'mosaic-tile-ci-test': { type: 'ci' },
      'mosaic-tile-ci-schema': { type: 'ci', needs: ['mosaic-tile-ci-format', 'mosaic-tile-ci-lint', 'mosaic-tile-ci-test'] },
      'deploy-mosaic-tile': { type: 'deploy', needs: ['mosaic-tile-ci-schema'], always_run: true },
      'mosaic-tile-e2e': { type: 'e2e', needs: ['deploy-mosaic-tile'] }
    }
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml'],
    jobs: {
      'mosaic-vending-ci-format': { type: 'ci' },
      'mosaic-vending-ci-lint': { type: 'ci' },
      'mosaic-vending-ci-test': { type: 'ci' },
      'mosaic-vending-ci-schema': { type: 'ci', needs: ['mosaic-vending-ci-format', 'mosaic-vending-ci-lint', 'mosaic-vending-ci-test'] },
      'deploy-mosaic-vending': { type: 'deploy', needs: ['mosaic-vending-ci-schema'], always_run: true },
      'mosaic-vending-e2e': { type: 'e2e', needs: ['deploy-mosaic-vending'] }
    }
  }
};

// Integration jobs that depend on multiple components
export const INTEGRATION_JOBS = {
  'full-e2e': {
    type: 'e2e',
    needs: [
      'frontend-ci-build',
      'mosaic-tile-e2e',
      'mosaic-vending-e2e'
    ]
  }
};

export function getAllFileNames(componentHashes) {
    alljobs = getAllJobs()
    return alljobs.flatMap((jobName, jobConfig) => {return jobName + componentsHashes[jobConfig.component] + ".json";});
}

// Helper to get all jobs for a component
export function getComponentJobs(component) {
  return COMPONENTS[component]?.jobs || {};
}

// Helper to get all jobs including integration jobs
export function getAllJobs() {
  const jobs = {};
  
  // Add component jobs
  Object.values(COMPONENTS).forEach((componentName, component) => {
    Object.entries(component.jobs).forEach(([jobName, jobConfig]) => {
      jobConfig['component'] = componentName
      jobs[jobName] = jobConfig;
    });
  });
  
  // Add integration jobs
  Object.entries(INTEGRATION_JOBS).forEach(([name, config]) => {
    config['component'] = 'integration'
    jobs[name] = config;
  });
  
  return jobs;
}

// Helper to check if a job should run based on dependencies
export function shouldJobRun(jobName, jobResults) {
  const allJobs = getAllJobs();
  const job = allJobs[jobName];
  
  if (!job) return false;
  
  // Always run deploy jobs if their dependencies pass
  if (job.always_run) {
    return job.needs?.every(need => jobResults[need]?.success) ?? true;
  }
  
  // For other jobs, check if any dependencies need to run or failed
  return job.needs?.some(need => {
    const needResult = jobResults[need];
    return !needResult?.exists || !needResult?.success;
  }) ?? true;
} 