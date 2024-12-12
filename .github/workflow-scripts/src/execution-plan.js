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

async function generateExecutionPlan() {
  const gistId = process.env.GIST_ID;
  const token = process.env.GIST_SECRET;
  const commitSha = process.env.GITHUB_SHA;

  if (!gistId || !token || !commitSha) {
    throw new Error('Missing required environment variables');
  }

  // Calculate component hashes first
  const componentHashes = {};
  Object.keys(COMPONENTS).forEach(componentName => {
    componentHashes[componentName] = calculateComponentHash(componentName);
  });

  // Get all possible filenames with hashes
  const allFileNames = getAllFileNames(componentHashes, commitSha);

  // Get all gist files
  const gistFiles = await getGistFiles(gistId, token);

  // Generate plan
  const plan = {
    components: componentHashes,
    jobs: {},
    metadata: {
      created_at: new Date().toISOString(),
      commit_sha: commitSha,
      run_id: process.env.GITHUB_RUN_ID,
      run_number: process.env.GITHUB_RUN_NUMBER,
      repository: process.env.GITHUB_REPOSITORY
    }
  };

  // Add all jobs with their previous run data
  Object.entries(getAllJobs()).forEach(([jobName, jobConfig]) => {
    // Find matching gist file for this job
    const filename = allFileNames.find(name => name.startsWith(jobName + '.'));
    const gistFile = filename ? gistFiles[filename] : null;

    let previousRun = null;
    if (gistFile) {
      try {
        const content = JSON.parse(gistFile.content);
        previousRun = {
          filename,
          content,
          exists: true,
          success: content.success,
          timestamp: content.timestamp,
          run_id: content.run_id,
          hash: filename.split('.')[1]?.replace('.json', '') || null
        };
      } catch (error) {
        console.warn(`Error parsing ${filename}: ${error.message}`);
      }
    }

    plan.jobs[jobName] = {
      component: jobConfig.component,
      needs_run: !previousRun?.success,
      previous_run: previousRun
    };
  });

  // Save plan (pretty print for file, compact for output)
  const planFile = join('.github', 'workflow-scripts', 'execution-plan.json');
  await writeFile(planFile, JSON.stringify(plan, null, 2));
  console.log('Generated execution plan:', JSON.stringify(plan));

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
