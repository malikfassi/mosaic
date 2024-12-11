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

  } catch (error) {
    console.error('Error in notify:', error);
    process.exit(1);
  }
}

main();