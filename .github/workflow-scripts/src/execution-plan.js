import { setOutput, setFailed } from '@actions/core';
import { context, getOctokit } from '@actions/github';
import { globSync } from 'glob';
import { createHash } from 'crypto';
import { readFileSync, existsSync } from 'fs';
import ignore from 'ignore';

// Component definitions
const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
    jobs: ['frontend-ci-lint', 'frontend-ci-test', 'frontend-ci-build']
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml'],
    jobs: [
      'mosaic-tile-ci-format',
      'mosaic-tile-ci-lint',
      'mosaic-tile-ci-test',
      'mosaic-tile-ci-schema',
      'deploy-mosaic-tile'
    ]
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml'],
    jobs: [
      'mosaic-vending-ci-format',
      'mosaic-vending-ci-lint',
      'mosaic-vending-ci-test',
      'mosaic-vending-ci-schema',
      'deploy-mosaic-vending'
    ]
  }
};

// E2E jobs use commit hash
const E2E_JOBS = [
  'mosaic-tile-e2e',
  'mosaic-vending-e2e',
  'full-e2e'
];

// Calculate hash for a component's files
function calculateComponentHash(component) {
  const hash = createHash('sha256');
  const { paths } = COMPONENTS[component];
  const ig = ignore();
  
  // Add root .gitignore if exists
  if (existsSync('.gitignore')) {
    ig.add(readFileSync('.gitignore', 'utf8'));
  }

  // Get all files matching the patterns
  const files = paths.flatMap(pattern => 
    globSync(pattern, { dot: true, nodir: true })
  ).filter(file => !ig.ignores(file));

  // Sort files for consistent hashing
  files.sort().forEach(file => {
    try {
      const content = readFileSync(file);
      hash.update(`${file}:`);
      hash.update(content);
    } catch (error) {
      console.warn(`Warning: Could not read file ${file}: ${error.message}`);
    }
  });

  return hash.digest('hex');
}

// Generate all filenames and their metadata
function generateFilenames() {
  const files = [];
  const components = {};
  
  // Generate component hashes and job files
  for (const [component, config] of Object.entries(COMPONENTS)) {
    const hash = calculateComponentHash(component);
    components[component] = { hash };
    
    // Add component job files
    for (const jobName of config.jobs) {
      files.push({
        filename: `${jobName}.${hash}.json`,
        jobName,
        componentName: component,
        componentHash: hash,
        isE2E: false
      });
    }
  }

  // Add E2E job files
  for (const jobName of E2E_JOBS) {
    // Determine component for E2E job
    let componentName = 'integration';
    if (jobName.startsWith('mosaic-tile-')) {
      componentName = 'mosaic_tile';
    } else if (jobName.startsWith('mosaic-vending-')) {
      componentName = 'mosaic_vending';
    }

    files.push({
      filename: `${jobName}.${context.sha}.json`,
      jobName,
      componentName,
      componentHash: context.sha,
      isE2E: true
    });
  }

  return { files, components };
}

async function generateExecutionPlan() {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;

    if (!token || !gistId) {
      throw new Error('Missing required environment variables');
    }

    // 1. Generate filenames and component data
    const { files, components } = generateFilenames();

    // 2. Fetch from gist
    const octokit = getOctokit(token);
    const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId }).catch(error => {
      if (error.status === 404) {
        console.warn('No gist found');
        return { data: { files: {} } };
      }
      throw error;
    });

    // 3. Generate execution plan from files
    const jobs = {};
    
    for (const fileInfo of files) {
      const file = gist.files[fileInfo.filename];
      let previous = null;

      if (file && file.content) {
        try {
          const content = JSON.parse(file.content);
          // Only use as previous if it was successful
          if (content.success) {
            previous = content;
          }
        } catch (error) {
          console.warn(`Error parsing ${fileInfo.filename}: ${error.message}`);
        }
      }

      const id = `${fileInfo.jobName}-${fileInfo.componentHash.substring(0, 8)}`;
      
      jobs[fileInfo.jobName] = {
        id,
        previous,
        results: {},
        componentName: fileInfo.componentName,
        componentHash: fileInfo.componentHash,
        success: false,
        workflowId: context.runId,
        commitId: context.sha,
        needs_run: !previous || !previous.success
      };
    }

    // Create complete execution plan
    const plan = {
      components,
      jobs,
      metadata: {
        created_at: new Date().toISOString(),
        commit_sha: context.sha,
        run_id: context.runId,
        run_number: context.runNumber,
        event_type: context.eventName,
        repository: `${context.repo.owner}/${context.repo.repo}`
      }
    };

    console.log('Generated plan:', JSON.stringify(plan, null, 2));
    setOutput('plan', JSON.stringify(plan));
  } catch (error) {
    console.error('Error generating execution plan:', error);
    setFailed(error.message);
  }
}

generateExecutionPlan(); 