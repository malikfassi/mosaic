import { getInput, setOutput, setFailed } from '@actions/core';
import { context, getOctokit } from '@actions/github';
import { globSync } from 'glob';
import { createHash } from 'crypto';
import { readFileSync, existsSync } from 'fs';
import ignore from 'ignore';

// Component base paths and their corresponding jobs
const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
    gitignorePath: 'frontend/.gitignore',
    jobs: ['frontend-ci']
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml', 'Cargo.toml'],
    gitignorePath: 'contracts/mosaic-tile-nft/.gitignore',
    jobs: ['mosaic-tile-ci', 'mosaic-tile-e2e']
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml', 'Cargo.toml'],
    gitignorePath: 'contracts/mosaic-vending-minter/.gitignore',
    jobs: ['mosaic-vending-ci', 'mosaic-vending-e2e']
  }
};

// Load and parse .gitignore files
function getIgnoreFilter(gitignorePath) {
  const ig = ignore();
  
  // Add common ignores
  ig.add(['node_modules', '.git', '*.log']);
  
  // Add root .gitignore if exists
  if (existsSync('.gitignore')) {
    ig.add(readFileSync('.gitignore', 'utf8'));
  }
  
  // Add component-specific .gitignore if exists
  if (existsSync(gitignorePath)) {
    ig.add(readFileSync(gitignorePath, 'utf8'));
  }
  
  return ig;
}

// Calculate hash for a component's files
function calculateComponentHash(component) {
  const hash = createHash('sha256');
  const { paths, gitignorePath } = COMPONENTS[component];
  const ig = getIgnoreFilter(gitignorePath);

  // Get all files matching the patterns
  const files = paths.flatMap(pattern => 
    globSync(pattern, { 
      dot: true,
      nodir: true
    })
  )
  // Filter using .gitignore rules
  .filter(file => !ig.ignores(file));

  // Sort files for consistent hashing
  files.sort().forEach(file => {
    try {
      const content = readFileSync(file);
      hash.update(`${file}:`); // Include filename in hash
      hash.update(content);
    } catch (error) {
      console.warn(`Warning: Could not read file ${file}: ${error.message}`);
    }
  });

  return hash.digest('hex');
}

// Check if job has a successful previous run
async function checkPreviousRun(octokit, jobName, componentHash) {
  try {
    const gistId = process.env.GIST_ID;
    if (!gistId) {
      console.warn('No GIST_ID provided');
      return { found: false };
    }

    // Get gist content
    const { data: gist } = await octokit.rest.gists.get({ 
      gist_id: gistId 
    });

    // Check if job result exists for this hash
    const filename = `${jobName}.${componentHash}.json`;
    if (filename in gist.files) {
      console.log(`Found previous run for ${jobName} with hash ${componentHash}`);
      return { found: true, hash: componentHash };
    }

    console.log(`No previous run found for ${jobName} with hash ${componentHash}`);
    return { found: false };
  } catch (error) {
    console.warn(`Warning: Error checking gist for ${jobName}: ${error.message}`);
    return { found: false };
  }
}

async function generateExecutionPlan() {
  try {
    const token = process.env.GIST_TOKEN;
    const gistId = process.env.GIST_ID;
    console.log('GIST_ID:', gistId); // Debug log
    const octokit = getOctokit(token);

    // Calculate component hashes
    const components = {};
    const jobResults = {};

    // First calculate all component hashes
    for (const [component, config] of Object.entries(COMPONENTS)) {
      const hash = calculateComponentHash(component);
      components[component] = { hash };
      
      // Check each job associated with this component
      for (const jobName of config.jobs) {
        const previousRun = await checkPreviousRun(octokit, jobName, hash);
        jobResults[jobName] = {
          needs_run: !previousRun.found,
          previous_run: previousRun
        };
      }
    }

    // Add deployment and E2E jobs
    const deployJobs = ['deploy-mosaic-tile', 'deploy-mosaic-vending'];
    for (const jobName of deployJobs) {
      jobResults[jobName] = {
        needs_run: true, // Always run deployments if CI passes
        previous_run: { found: false }
      };
    }

    // Full E2E needs to run if any component changed
    jobResults['full-e2e'] = {
      needs_run: Object.values(components).some(c => 
        Object.values(jobResults).some(j => j.needs_run)
      ),
      previous_run: { found: false }
    };

    // Create complete execution plan
    const plan = {
      components,
      jobs: Object.fromEntries(
        Object.entries(jobResults).map(([name, data]) => [
          name,
          { needs_run: data.needs_run.toString() }
        ])
      ),
      metadata: {
        created_at: new Date().toISOString(),
        commit_sha: context.sha,
        run_id: context.runId,
        run_number: context.runNumber,
        event_type: context.eventName,
        repository: `${context.repo.owner}/${context.repo.repo}`,
        gist_id: gistId
      }
    };

    console.log('Generated plan:', JSON.stringify(plan, null, 2));
    setOutput('plan', JSON.stringify(plan));
  } catch (error) {
    console.error('Error generating execution plan:', error);
    console.error('Environment:', {
      GIST_ID: process.env.GIST_ID,
      GITHUB_TOKEN: !!process.env.GITHUB_TOKEN
    });
    setFailed(error.message);
  }
}

generateExecutionPlan(); 