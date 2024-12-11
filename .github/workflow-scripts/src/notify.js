import { writeFile } from 'fs/promises';
import { getOctokit } from '@actions/github';
import { JOBS, COMPONENTS, getAllFileNames } from './workflow-config.js';

function formatJobResult(result) {
  if (!result) return '⚪️ Not run';
  return result === 'success' ? '✅ Success' : '❌ Failed';
}

function generateStatusMessage(plan, results) {
  const lines = [];

  // Frontend status
  if (COMPONENTS.frontend) {
    lines.push('**Frontend**');
    COMPONENTS.frontend.jobs.forEach(jobName => {
      const previousRun = plan.jobs[jobName]?.previous_run;
      lines.push(`- ${jobName}: ${formatJobResult(results[jobName])}${previousRun ? ` (Previous: ${formatJobResult(previousRun.result)})` : ''}`);
    });
  }

  // Mosaic Tile status
  if (COMPONENTS.mosaic_tile) {
    lines.push('**Mosaic Tile**');
    COMPONENTS.mosaic_tile.jobs.forEach(jobName => {
      const previousRun = plan.jobs[jobName]?.previous_run;
      lines.push(`- ${jobName}: ${formatJobResult(results[jobName])}${previousRun ? ` (Previous: ${formatJobResult(previousRun.result)})` : ''}`);
    });
  }

  // Mosaic Vending status
  if (COMPONENTS.mosaic_vending) {
    lines.push('**Mosaic Vending**');
    COMPONENTS.mosaic_vending.jobs.forEach(jobName => {
      const previousRun = plan.jobs[jobName]?.previous_run;
      lines.push(`- ${jobName}: ${formatJobResult(results[jobName])}${previousRun ? ` (Previous: ${formatJobResult(previousRun.result)})` : ''}`);
    });
  }

  // Integration status (full-e2e)
  if (plan.jobs[JOBS.FULL_E2E]) {
    lines.push('**Integration**');
    const previousRun = plan.jobs[JOBS.FULL_E2E]?.previous_run;
    lines.push(`- ${JOBS.FULL_E2E}: ${formatJobResult(results[JOBS.FULL_E2E])}${previousRun ? ` (Previous: ${formatJobResult(previousRun.result)})` : ''}`);
  }

  return lines.join('\n');
}

async function main() {
  try {
    // Get inputs from environment
    const plan = JSON.parse(process.env.EXECUTION_PLAN);
    const results = {
      [JOBS.FRONTEND_LINT]: process.env.FRONTEND_CI_RESULT,
      [JOBS.FRONTEND_TEST]: process.env.FRONTEND_CI_RESULT,
      [JOBS.FRONTEND_BUILD]: process.env.FRONTEND_CI_RESULT,
      [JOBS.MOSAIC_TILE_FORMAT]: process.env.MOSAIC_TILE_CI_RESULT,
      [JOBS.MOSAIC_TILE_LINT]: process.env.MOSAIC_TILE_CI_RESULT,
      [JOBS.MOSAIC_TILE_TEST]: process.env.MOSAIC_TILE_CI_RESULT,
      [JOBS.MOSAIC_TILE_SCHEMA]: process.env.MOSAIC_TILE_CI_RESULT,
      [JOBS.MOSAIC_TILE_DEPLOY]: process.env.MOSAIC_TILE_DEPLOY_RESULT,
      [JOBS.MOSAIC_TILE_E2E]: process.env.MOSAIC_TILE_E2E_RESULT,
      [JOBS.MOSAIC_VENDING_FORMAT]: process.env.MOSAIC_VENDING_CI_RESULT,
      [JOBS.MOSAIC_VENDING_LINT]: process.env.MOSAIC_VENDING_CI_RESULT,
      [JOBS.MOSAIC_VENDING_TEST]: process.env.MOSAIC_VENDING_CI_RESULT,
      [JOBS.MOSAIC_VENDING_SCHEMA]: process.env.MOSAIC_VENDING_CI_RESULT,
      [JOBS.MOSAIC_VENDING_DEPLOY]: process.env.MOSAIC_VENDING_DEPLOY_RESULT,
      [JOBS.MOSAIC_VENDING_E2E]: process.env.MOSAIC_VENDING_E2E_RESULT,
      [JOBS.FULL_E2E]: process.env.FULL_E2E_RESULT
    };

    // Generate Discord message
    const message = [
      '🔄 **Mosaic CI/CD Status**\n',
      generateStatusMessage(plan, results),
      '',
      `[View run](${plan.metadata.repository}/actions/runs/${plan.metadata.run_id})`
    ].join('\n');

    // Save Discord message
    await writeFile('discord_message.txt', message);

    // Update gist with data from results (only update if successful)
    const octokit = getOctokit(process.env.GIST_SECRET);
    await octokit.rest.gists.update({
      gist_id: process.env.GIST_ID,
      // for all successful jobs, filename must be jobname and hash
      // content must be the metadata from the job and the previous job
      
      files: Object.entries(results).filter(([_, result]) => result === 'success').reduce((acc, [key, _]) => {
        const job = plan.jobs[key];
        const previousRun = job.previous_run || {};
        const content = {
          success: true,
          timestamp: new Date().toISOString(),
          run_id: plan.metadata.run_id,
          hash: job.hash,
          previous_run: {
            success: previousRun.success,
            timestamp: previousRun.timestamp,
            run_id: previousRun.run_id,
            hash: previousRun.hash
          }
        };
        acc[`${key}.${job.hash}.json`] = { content: JSON.stringify(content, null, 2) };
        return acc;
      }, {})
    });

  } catch (error) {
    console.error('Error in notify:', error);
    process.exit(1);
  }

}

await main();