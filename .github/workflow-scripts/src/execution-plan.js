import { getInput, setOutput, setFailed } from '@actions/core';
import { context, getOctokit } from '@actions/github';
import { globSync } from 'glob';
import { createHash } from 'crypto';
import { readFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import ignore from 'ignore';

// Component base paths and their corresponding jobs
const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
    gitignorePath: '.gitignore',   
    jobs: ['frontend-lint', 'frontend-test']
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml', 'Cargo.toml'],
    gitignorePath: '.gitignore',
    jobs: ['mosaic-tile-format', 'mosaic-tile-lint', 'mosaic-tile-test', 'mosaic-tile-schema', 'deploy-mosaic-tile', 'mosaic-tile-e2e']
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml', 'Cargo.toml'],
    gitignorePath: '.gitignore',
    jobs: ['mosaic-vending-format', 'mosaic-vending-lint', 'mosaic-vending-test', 'mosaic-vending-schema', 'deploy-mosaic-vending', 'mosaic-vending-e2e']
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

// Check if job needs to run based on previous successful run in gist
async function checkNeedsRun(octokit, jobName, componentHash) {
  try {
    const gistId = process.env.GIST_ID;
    if (!gistId) {
      console.warn('No GIST_ID provided, will run all jobs');
      return true;
    }

    // Get gist content
    const { data: gist } = await octokit.rest.gists.get({ 
      gist_id: gistId 
    });

    // Check if job result exists for this hash
    const filename = `${jobName}.${componentHash}.json`;
    if (filename in gist.files) {
      console.log(`Found successful previous run for ${jobName} with matching hash`);
      return false;
    }

    console.log(`No previous successful run found for ${jobName} with hash ${componentHash}`);
  } catch (error) {
    console.warn(`Warning: Error checking gist for ${jobName}: ${error.message}`);
  }

  return true; // Run by default if any errors or no matching run found
}

async function generateExecutionPlan() {
  try {
    const token = process.env.GITHUB_TOKEN;
    const octokit = getOctokit(token);

    // Calculate component hashes and job needs
    const components = {};
    const jobResults = {};

    // First calculate all component hashes
    for (const [component, config] of Object.entries(COMPONENTS)) {
      const hash = calculateComponentHash(component);
      components[component] = { hash };
      
      // Check each job associated with this component
      for (const jobName of config.jobs) {
        const needs_run = await checkNeedsRun(octokit, jobName, hash);
        jobResults[jobName] = { needs_run };
      }
    }

    // Generate job conditions
    const jobs = {
      'frontend-ci': {
        needs_run: jobResults['frontend-ci'].needs_run.toString()
      },
      'mosaic-tile-ci': {
        needs_run: jobResults['mosaic-tile-ci'].needs_run.toString()
      },
      'mosaic-vending-ci': {
        needs_run: jobResults['mosaic-vending-ci'].needs_run.toString()
      },
      'deploy-mosaic-tile': {
        needs_run: jobResults['mosaic-tile-ci'].needs_run.toString(),
      },
      'deploy-mosaic-vending': {
        needs_run: jobResults['mosaic-vending-ci'].needs_run.toString(),
      },
      'mosaic-tile-e2e': {
        needs_run: jobResults['mosaic-tile-e2e'].needs_run.toString()
      },
      'mosaic-vending-e2e': {
        needs_run: jobResults['mosaic-vending-e2e'].needs_run.toString()
      },
      'full-e2e': {
        needs_run: (Object.values(jobResults).some(j => j.needs_run)).toString()
      }
    };

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

    setOutput('plan', JSON.stringify(plan));
  } catch (error) {
    setFailed(error.message);
  }
}

generateExecutionPlan(); 