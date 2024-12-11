import { getOctokit } from '@actions/github';

// Map components to their jobs
const COMPONENT_JOBS = {
  frontend: ['frontend-ci-lint', 'frontend-ci-test', 'frontend-ci-build'],
  mosaic_tile: [
    'mosaic-tile-ci-format',
    'mosaic-tile-ci-lint',
    'mosaic-tile-ci-test',
    'mosaic-tile-ci-schema',
    'deploy-mosaic-tile',
    'mosaic-tile-e2e'
  ],
  mosaic_vending: [
    'mosaic-vending-ci-format',
    'mosaic-vending-ci-lint',
    'mosaic-vending-ci-test',
    'mosaic-vending-ci-schema',
    'deploy-mosaic-vending',
    'mosaic-vending-e2e'
  ],
  full: ['full-e2e']
};

// Get status for a specific component hash
async function getComponentStatus(octokit, gistId, componentHash) {
  try {
    // Get gist content - we can't filter files, but we can optimize processing
    const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId });

    // Get all jobs for this component
    const jobs = COMPONENT_JOBS[componentHash.component] || [];
    const results = {};

    // Check each job file
    for (const job of jobs) {
      const filename = `${job}.${componentHash.hash}.json`;
      const file = gist.files[filename];
      
      if (file && file.content) {
        try {
          const content = JSON.parse(file.content);
          results[job] = {
            exists: true,
            success: content.success || false,
            timestamp: content.timestamp || null,
            run_id: content.run_id || null
          };
        } catch (error) {
          console.warn(`Error parsing ${filename}: ${error.message}`);
          results[job] = { exists: false, success: false, timestamp: null };
        }
      } else {
        results[job] = { exists: false };
      }
    }

    return {
      component: componentHash.component,
      hash: componentHash.hash,
      jobs: results
    };
  } catch (error) {
    if (error.status === 404) {
      console.warn(`No gist found for ${componentHash.component}`);
      return {
        component: componentHash.component,
        hash: componentHash.hash,
        jobs: Object.fromEntries(jobs.map(job => [job, { exists: false }]))
      };
    }
    
    console.error(`Error getting status for ${componentHash.component}: ${error.message}`);
    return {
      component: componentHash.component,
      hash: componentHash.hash,
      jobs: {},
      error: error.message
    };
  }
}

// Main function to get all statuses
async function getStatus() {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;
    const executionPlan = JSON.parse(process.env.EXECUTION_PLAN || '{}');

    if (!token || !gistId) {
      throw new Error('Missing required environment variables');
    }

    const octokit = getOctokit(token);
    const statuses = [];

    // Get status for each component in the execution plan
    for (const [component, data] of Object.entries(executionPlan.components || {})) {
      const status = await getComponentStatus(octokit, gistId, {
        component,
        hash: data.hash
      });
      statuses.push(status);
    }

    return statuses;
  } catch (error) {
    console.error('Error getting statuses:', error);
    throw error;
  }
}

export { getStatus, getComponentStatus }; 