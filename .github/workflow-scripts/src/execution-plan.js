import { writeFile } from 'fs/promises';
import { getOctokit } from '@actions/github';
import { createHash } from 'crypto';
import { globSync } from 'glob';
import { readFileSync, existsSync } from 'fs';
import ignore from 'ignore';
import { getAllJobs, COMPONENTS, getAllFileNames } from './workflow-config.js';
import { dirname, join } from 'path';

// Change to workspace root (two levels up from script location)
const scriptDir = dirname(new URL(import.meta.url).pathname);
process.chdir(join(scriptDir, '..', '..', '..'));

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
  const files = paths.flatMap(pattern => {
    const matches = globSync(pattern, { 
      dot: true, 
      nodir: true,
      cwd: process.cwd(),
      absolute: false
    });
    console.log(`Found ${matches.length} files for pattern ${pattern}`);
    return matches;
  }).filter(file => !ig.ignores(file));

  if (files.length === 0) {
    console.warn(`No files found for component ${component} with patterns:`, paths);
  } else {
    console.log(`Found ${files.length} files for component ${component}:`);
  }

  // Sort files for consistent hashing
  files.sort().forEach(file => {
    try {
      const content = readFileSync(file, 'utf8');
      hash.update(`${file}:`);
      hash.update(content);
    } catch (error) {
      console.warn(`Warning: Could not read file ${file}: ${error.message}`);
    }
  });

  const result = hash.digest('hex');
  console.log(`Hash for ${component}: ${result}`);
  return result;
}

async function getGistFiles(gistId, token) {
  const octokit = getOctokit(token);
  const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId }).catch(error => {
    if (error.status === 404) {
      console.warn('No gist found');
      return { data: { files: {} } };
    }
    throw error;
  });

  return gist.files;
}

// Add this function to get previous run data
async function getPreviousRunData(octokit, owner, repo, jobName) {
  try {
    const response = await octokit.rest.actions.listWorkflowRuns({
      owner,
      repo,
      workflow_id: 'mosaic.yml',
      status: 'completed',
      per_page: 1
    });

    if (response.data.workflow_runs.length > 0) {
      const previousRun = response.data.workflow_runs[0];
      return {
        run_id: previousRun.id,
        success: previousRun.conclusion === 'success'
      };
    }
    return null;
  } catch (error) {
    console.warn(`Failed to get previous run data for ${jobName}:`, error);
    return null;
  }
}

// Modify the generateExecutionPlan function
async function generateExecutionPlan(octokit, owner, repo, componentHashes, commit_sha) {
  const plan = {
    metadata: {
      repository: `${owner}/${repo}`,
      run_id: process.env.GITHUB_RUN_ID,
      commit_sha
    },
    components: componentHashes,
    jobs: {}
  };

  // Get all jobs
  const allJobs = getAllJobs();

  // For each job, determine if it needs to run and get previous run data
  for (const [jobName, jobConfig] of Object.entries(allJobs)) {
    const previousRun = await getPreviousRunData(octokit, owner, repo, jobName);
    
    plan.jobs[jobName] = {
      needs_run: needsToRun(jobName, jobConfig.component, componentHashes),
      previous_run: previousRun,
      component: jobConfig.component,
      component_hash: componentHashes[jobConfig.component]
    };
  }

  return plan;
}

async function main() {
  try {
    await generateExecutionPlan();
  } catch (error) {
    console.error('Error generating execution plan:', error);
    process.exit(1);
  }
}

main();
