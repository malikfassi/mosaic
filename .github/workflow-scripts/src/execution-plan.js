import { writeFile } from 'fs/promises';
import { getOctokit } from '@actions/github';
import { createHash } from 'crypto';
import { globSync } from 'glob';
import { readFileSync, existsSync } from 'fs';
import ignore from 'ignore';
import { getAllJobs, COMPONENTS, HASH_JOBS } from './workflow-config.js';

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

async function checkGistFiles(gistId, token) {
  const octokit = getOctokit(token);
  const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId }).catch(error => {
    if (error.status === 404) {
      console.warn('No gist found');
      return { data: { files: {} } };
    }
    throw error;
  });

  // Get previous run data from gist files
  const previousRuns = {};
  Object.entries(gist.files).forEach(([filename, file]) => {
    try {
      const content = JSON.parse(file.content);
      const jobName = filename.split('.')[0];
      previousRuns[jobName] = {
        exists: true,
        success: content.success,
        timestamp: content.timestamp,
        run_id: content.run_id
      };
    } catch (error) {
      console.warn(`Error parsing ${filename}: ${error.message}`);
    }
  });

  return previousRuns;
}

async function generateExecutionPlan() {
  const gistId = process.env.GIST_ID;
  const token = process.env.GIST_SECRET;

  if (!gistId || !token) {
    throw new Error('Missing required environment variables');
  }

  // Get previous runs from gist
  const previousRuns = await checkGistFiles(gistId, token);

  // Calculate component hashes and check gist existence
  const components = {};
  Object.keys(COMPONENTS).forEach(componentName => {
    const hash = calculateComponentHash(componentName);
    components[componentName] = {
      hash,
      gist_exists: Object.values(previousRuns).some(run => run.exists && run.success)
    };
  });

  // Generate plan
  const plan = {
    components,
    jobs: {},
    metadata: {
      created_at: new Date().toISOString(),
      commit_sha: process.env.GITHUB_SHA,
      run_id: process.env.GITHUB_RUN_ID,
      run_number: process.env.GITHUB_RUN_NUMBER,
      repository: process.env.GITHUB_REPOSITORY
    }
  };

  // Add all jobs
  Object.entries(getAllJobs()).forEach(([jobName, jobConfig]) => {
    const component = components[jobConfig.component];
    const previousRun = previousRuns[jobName];
    plan.jobs[jobName] = {
      component: jobConfig.component,
      needs_run: !previousRun?.exists || !previousRun?.success,
      previous_run: previousRun
    };
  });

  // Save plan
  const planJson = JSON.stringify(plan);
  await writeFile('execution-plan.json', planJson);
  console.log('Generated execution plan:', planJson);

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
