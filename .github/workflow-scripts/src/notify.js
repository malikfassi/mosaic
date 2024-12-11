import * as core from '@actions/core';
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
    const formatComponent = (component, jobs) => {
      const componentInfo = plan.components[component];
      if (!componentInfo) return '';

      const jobsList = jobs.map(jobName => {
        const jobInfo = plan.jobs[jobName];
        if (!jobInfo) return '';

        const wasReused = jobInfo.previous && jobInfo.previous.success;
        const status = wasReused 
          ? '✅ _(reused)_'
          : formatJobResult(results[jobName.replace(/-/g, '_')]);

        return `${jobName}: ${status}`;
      }).filter(Boolean);

      if (jobsList.length === 0) return '';

      return `**${component}**\n${jobsList.join('\n')}`;
    };

    // Group jobs by component
    const componentJobs = {
      frontend: [
        'frontend-ci-lint',
        'frontend-ci-test',
        'frontend-ci-build'
      ],
      mosaic_tile: [
        'mosaic-tile-ci-format',
        'mosaic-tile-ci-lint',
        'mosaic-tile-ci-test',
        'mosaic-tile-ci-schema',
        'deploy-mosaic-tile',
        'mosaic-tile-e2e'
      ],
      mosaic_vending: [
        'mosaic-vending-ci-format',
        'mosaic-vending-ci-lint',
        'mosaic-vending-ci-test',
        'mosaic-vending-ci-schema',
        'deploy-mosaic-vending',
        'mosaic-vending-e2e'
      ]
    };

    // Create message sections
    const sections = [];

    // Add component sections
    for (const [component, jobs] of Object.entries(componentJobs)) {
      const section = formatComponent(component, jobs);
      if (section) sections.push(section);
    }

    // Add integration section
    if (plan.jobs['full-e2e']) {
      sections.push(
        '**Integration**',
        `full-e2e: ${formatJobResult(results.full_e2e)}`
      );
    }

    // Create message
    const message = [
      '🔄 **Mosaic CI/CD Status**\n',
      ...sections,
      '',
      `[View run](${plan.metadata.repository}/actions/runs/${plan.metadata.run_id})`
    ].join('\n');

    // Save message to file
    await writeFile('discord_message.txt', message);

  } catch (error) {
    core.setFailed(error.message);
  }
}

main(); 