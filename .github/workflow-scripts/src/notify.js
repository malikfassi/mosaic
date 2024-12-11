import * as core from '@actions/core';
import * as github from '@actions/github';
import { writeFile } from 'fs/promises';

function formatJobResult(result) {
  switch (result) {
    case 'success': return '✅';
    case 'failure': return '❌';
    case 'cancelled': return '⚪';
    case 'skipped': return '⏭️';
    default: return '❓';
  }
}

async function main() {
  try {
    // Get inputs
    const plan = JSON.parse(core.getInput('execution_plan', { required: true }));
    const results = {
      frontend_ci: core.getInput('frontend_ci_result', { required: true }),
      mosaic_tile_ci: core.getInput('mosaic_tile_ci_result', { required: true }),
      mosaic_vending_ci: core.getInput('mosaic_vending_ci_result', { required: true }),
      mosaic_tile_deploy: core.getInput('mosaic_tile_deploy_result', { required: true }),
      mosaic_vending_deploy: core.getInput('mosaic_vending_deploy_result', { required: true }),
      mosaic_tile_e2e: core.getInput('mosaic_tile_e2e_result', { required: true }),
      mosaic_vending_e2e: core.getInput('mosaic_vending_e2e_result', { required: true }),
      full_e2e: core.getInput('full_e2e_result', { required: true })
    };

    // Format component sections
    const formatComponent = (component, changes, pipeline) => {
      let changesText;
      if (component.files_changed) {
        changesText = '📝 Changes:\n' + component.changed_files.map(f => `- ${f}`).join('\n');
      } else if (component.previous_run.found) {
        const { timestamp, url } = component.previous_run.run;
        const date = timestamp.split('T')[0];
        changesText = `✨ No changes _(using ${date} [run](${url}))_`;
      } else {
        changesText = '🆕 First run';
      }

      return `**${component.name}**\n${changesText}\n${pipeline}`;
    };

    // Frontend section
    const frontendPipeline = plan.jobs['frontend-ci'].needs_run
      ? `CI: ${formatJobResult(results.frontend_ci)}`
      : '✅ CI _(reused)_';

    const frontendSection = formatComponent(
      plan.components.frontend,
      plan.components.frontend.changed_files,
      frontendPipeline
    );

    // Mosaic Tile section
    let mosaicTilePipeline = '';
    if (!plan.jobs['mosaic-tile-ci'].needs_run) {
      mosaicTilePipeline += '✅ CI _(reused)_ → ';
    } else {
      mosaicTilePipeline += `CI: ${formatJobResult(results.mosaic_tile_ci)} → `;
    }
    if (!plan.jobs['deploy-mosaic-tile'].needs_run) {
      mosaicTilePipeline += '✅ Deploy _(reused)_ → ';
    } else {
      mosaicTilePipeline += `Deploy: ${formatJobResult(results.mosaic_tile_deploy)} → `;
    }
    if (!plan.jobs['mosaic-tile-e2e'].needs_run) {
      mosaicTilePipeline += '✅ E2E _(reused)_';
    } else {
      mosaicTilePipeline += `E2E: ${formatJobResult(results.mosaic_tile_e2e)}`;
    }

    const mosaicTileSection = formatComponent(
      plan.components.mosaic_tile,
      plan.components.mosaic_tile.changed_files,
      mosaicTilePipeline
    );

    // Mosaic Vending section
    let mosaicVendingPipeline = '';
    if (!plan.jobs['mosaic-vending-ci'].needs_run) {
      mosaicVendingPipeline += '✅ CI _(reused)_ → ';
    } else {
      mosaicVendingPipeline += `CI: ${formatJobResult(results.mosaic_vending_ci)} → `;
    }
    if (!plan.jobs['deploy-mosaic-vending'].needs_run) {
      mosaicVendingPipeline += '✅ Deploy _(reused)_ → ';
    } else {
      mosaicVendingPipeline += `Deploy: ${formatJobResult(results.mosaic_vending_deploy)} → `;
    }
    if (!plan.jobs['mosaic-vending-e2e'].needs_run) {
      mosaicVendingPipeline += '✅ E2E _(reused)_';
    } else {
      mosaicVendingPipeline += `E2E: ${formatJobResult(results.mosaic_vending_e2e)}`;
    }

    const mosaicVendingSection = formatComponent(
      plan.components.mosaic_vending,
      plan.components.mosaic_vending.changed_files,
      mosaicVendingPipeline
    );

    // Full E2E section
    const fullE2eStatus = plan.jobs['full-e2e'].needs_run
      ? `Full E2E: ${formatJobResult(results.full_e2e)}`
      : '✅ Full E2E _(reused)_';

    // Create message
    const message = [
      '🔄 **Mosaic CI/CD Status**\n',
      frontendSection,
      '',
      mosaicTileSection,
      '',
      mosaicVendingSection,
      '',
      '**Integration**',
      fullE2eStatus,
      '',
      `[View run](${plan.metadata.repository}/actions/runs/${plan.metadata.workflow_id})`
    ].join('\n');

    // Save message to file
    await writeFile('discord_message.txt', message);

  } catch (error) {
    core.setFailed(error.message);
  }
}

main(); 