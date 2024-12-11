import * as core from '@actions/core';
import * as github from '@actions/github';
import { loadFromCache, saveToCache, clearExpiredCache } from './cache.js';

async function getGistFiles(octokit, gistId, componentHash) {
  // Try to get data from cache first
  const cachedData = await loadFromCache(gistId, componentHash);
  if (cachedData) {
    core.info('Using cached gist data');
    return cachedData;
  }

  // If not in cache, fetch from GitHub
  core.info('Fetching gist data from GitHub');
  try {
    const gist = await octokit.rest.gists.get({ gist_id: gistId });
    const files = gist.data.files;
    
    // Save to cache for future use
    await saveToCache(gistId, componentHash, files);
    
    return files;
  } catch (error) {
    core.warning(`Error getting gist: ${error.message}`);
    return {};
  }
}

async function main() {
  try {
    // Clear expired cache entries at the start
    await clearExpiredCache();

    // Get inputs
    const action = core.getInput('action', { required: true });
    const gistId = core.getInput('gist_id', { required: true });
    const gistToken = core.getInput('gist_token', { required: true });
    const componentName = core.getInput('component_name');
    const componentHash = core.getInput('component_hash');
    const executionPlan = core.getInput('execution_plan');

    const octokit = github.getOctokit(gistToken);

    if (action === 'read') {
      // Find previous successful run for component
      if (componentName && componentHash) {
        // Get gist files from cache or GitHub
        const files = await getGistFiles(octokit, gistId, componentHash);

        // Look for job result files for this component hash
        const jobFiles = Object.entries(files)
          .filter(([filename]) => filename.endsWith(`.${componentHash}.json`))
          .map(([_, file]) => {
            try {
              return JSON.parse(file.content);
            } catch {
              return null;
            }
          })
          .filter(Boolean);

        // Find the most recent successful run
        const previousRun = jobFiles
          .filter(job => job.result === 'success')
          .sort((a, b) => new Date(b.date) - new Date(a.date))[0];

        const result = previousRun
          ? { found: true, run: previousRun }
          : { found: false };

        core.setOutput('previous_run', JSON.stringify(result));
      }

    } else if (action === 'update' && executionPlan) {
      const plan = JSON.parse(executionPlan);
      const { metadata } = plan;

      // Create job result files for each component
      const files = {};
      for (const [componentName, component] of Object.entries(plan.components)) {
        // Get all jobs for this component
        const componentJobs = Object.entries(plan.jobs)
          .filter(([_, job]) => job.component === componentName);

        // Create a file for each job
        for (const [jobName, job] of componentJobs) {
          if (!job.needs_run) continue; // Skip jobs that weren't run

          const filename = `${jobName}.${component.hash}.json`;
          const jobResult = {
            job_name: jobName,
            component_name: componentName,
            component_hash: component.hash,
            commit_hash: metadata.commit_sha,
            run_id: metadata.workflow_id,
            result: job.previous_result ? 'success' : 'failure',
            date: metadata.created_at,
            changed_files: component.changed_files,
            url: `https://github.com/${metadata.repository}/actions/runs/${metadata.workflow_id}`
          };

          files[filename] = {
            content: JSON.stringify(jobResult, null, 2)
          };

          // Save each job result to cache
          await saveToCache(gistId, component.hash, {
            [filename]: files[filename]
          });
        }
      }

      // Update gist with new files
      await octokit.rest.gists.update({
        gist_id: gistId,
        files
      });

      core.setOutput('status_data', JSON.stringify({ files }));
    }

  } catch (error) {
    core.setFailed(error.message);
  }
}

main(); 