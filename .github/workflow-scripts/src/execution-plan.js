import { getInput, setOutput, setFailed } from '@actions/core';
import { context, getOctokit } from '@actions/github';
import { globSync } from 'glob';
import { createHash } from 'crypto';
import { readFileSync, existsSync } from 'fs';
import ignore from 'ignore';
import { getComponentStatus } from './get-status.js';

// Component base paths
const COMPONENTS = {
  frontend: {
    paths: ['frontend/**/*'],
  },
  mosaic_tile: {
    paths: ['contracts/mosaic-tile-nft/**/*', 'contracts/Cargo.toml'],
  },
  mosaic_vending: {
    paths: ['contracts/mosaic-vending-minter/**/*', 'contracts/Cargo.toml'],
  }
};

// Load and parse .gitignore files
function getIgnoreFilter() {
  const ig = ignore();
  
  // Add root .gitignore if exists
  if (existsSync('.gitignore')) {
    ig.add(readFileSync('.gitignore', 'utf8'));
  }
  
  return ig;
}

// Calculate hash for a component's files
function calculateComponentHash(component) {
  const hash = createHash('sha256');
  const { paths } = COMPONENTS[component];
  const ig = getIgnoreFilter();

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

async function generateExecutionPlan() {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;
    console.log('GIST_ID:', gistId);
    const octokit = getOctokit(token);

    // Calculate component hashes
    const components = {};
    const jobResults = {};

    // First calculate all component hashes and get their statuses
    for (const [component, config] of Object.entries(COMPONENTS)) {
      const hash = calculateComponentHash(component);
      components[component] = { hash };
      
      // Get status for this component
      const status = await getComponentStatus(octokit, gistId, { component, hash });
      
      // Check each job's status
      for (const [jobName, result] of Object.entries(status.jobs)) {
        jobResults[jobName] = {
          needs_run: !result.exists || !result.success,
          previous_run: {
            found: result.exists && result.success,
            run_id: result.run_id,
            timestamp: result.timestamp
          }
        };
      }
    }

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
    setFailed(error.message);
  }
}

generateExecutionPlan(); 