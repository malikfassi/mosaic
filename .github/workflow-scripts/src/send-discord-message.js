import fetch from 'node-fetch';
import { JOBS } from './workflow-config.js';

function formatJobStatus(jobName, result) {
  const emoji = result === 'success' ? '✅' : result === 'skipped' ? '⏭️' : '❌';
  return `${emoji} ${jobName}: ${result}`;
}

function getJobsByComponent(component) {
  return Object.entries(JOBS)
    .filter(([_, config]) => config.component.name === component)
    .map(([jobName]) => jobName);
}

function generateDiscordMessage(planResults) {
  const { metadata, jobs } = planResults;
  
  // Build message sections
  const sections = [];

  // Header
  sections.push(`**Workflow Run: ${metadata.workflow_id}**`);
  sections.push(`Branch: \`${metadata.branch}\``);
  sections.push(`Commit: \`${metadata.commit_sha.substring(0, 7)}\``);
  sections.push('');

  // Frontend section
  const frontendJobs = getJobsByComponent('frontend');
  if (frontendJobs.some(job => jobs[job])) {
    sections.push('**Frontend**');
    if (jobs.frontend_lint) {
      sections.push(formatJobStatus('Lint', jobs.frontend_lint.result));
    }
    if (jobs.frontend_test) {
      sections.push(formatJobStatus('Test', jobs.frontend_test.result));
    }
    if (jobs.frontend_build) {
      sections.push(formatJobStatus('Build', jobs.frontend_build.result));
    }
    sections.push('');
  }

  // Mosaic Tile section
  const tileJobs = getJobsByComponent('mosaic_tile');
  if (tileJobs.some(job => jobs[job])) {
    sections.push('**Mosaic Tile**');
    if (jobs.mosaic_tile_nft_fmt) {
      sections.push(formatJobStatus('Format', jobs.mosaic_tile_nft_fmt.result));
    }
    if (jobs.mosaic_tile_nft_clippy) {
      sections.push(formatJobStatus('Lint', jobs.mosaic_tile_nft_clippy.result));
    }
    if (jobs.mosaic_tile_nft_test) {
      sections.push(formatJobStatus('Test', jobs.mosaic_tile_nft_test.result));
    }
    if (jobs.mosaic_tile_nft_compile) {
      sections.push(formatJobStatus('Compile', jobs.mosaic_tile_nft_compile.result));
    }
    if (jobs.mosaic_tile_nft_deploy) {
      sections.push(formatJobStatus('Deploy', jobs.mosaic_tile_nft_deploy.result));
      if (jobs.mosaic_tile_nft_deploy.data && jobs.mosaic_tile_nft_deploy.data.code_id) {
        sections.push(`Code ID: \`${jobs.mosaic_tile_nft_deploy.data.code_id}\``);
      }
      if (jobs.mosaic_tile_nft_deploy.data && jobs.mosaic_tile_nft_deploy.data.contract_address) {
        sections.push(`Contract: \`${jobs.mosaic_tile_nft_deploy.data.contract_address}\``);
      }
    }
    if (jobs.mosaic_tile_nft_e2e) {
      sections.push(formatJobStatus('E2E Tests', jobs.mosaic_tile_nft_e2e.result));
    }
    sections.push('');
  }

  // Mosaic Vending section
  const vendingJobs = getJobsByComponent('mosaic_vending');
  if (vendingJobs.some(job => jobs[job])) {
    sections.push('**Mosaic Vending**');
    if (jobs.mosaic_vending_minter_fmt) {
      sections.push(formatJobStatus('Format', jobs.mosaic_vending_minter_fmt.result));
    }
    if (jobs.mosaic_vending_minter_clippy) {
      sections.push(formatJobStatus('Lint', jobs.mosaic_vending_minter_clippy.result));
    }
    if (jobs.mosaic_vending_minter_test) {
      sections.push(formatJobStatus('Test', jobs.mosaic_vending_minter_test.result));
    }
    if (jobs.mosaic_vending_minter_compile) {
      sections.push(formatJobStatus('Compile', jobs.mosaic_vending_minter_compile.result));
    }
    if (jobs.mosaic_vending_minter_deploy) {
      sections.push(formatJobStatus('Deploy', jobs.mosaic_vending_minter_deploy.result));
      if (jobs.mosaic_vending_minter_deploy.data && jobs.mosaic_vending_minter_deploy.data.code_id) {
        sections.push(`Code ID: \`${jobs.mosaic_vending_minter_deploy.data.code_id}\``);
      }
      if (jobs.mosaic_vending_minter_deploy.data && jobs.mosaic_vending_minter_deploy.data.contract_address) {
        sections.push(`Contract: \`${jobs.mosaic_vending_minter_deploy.data.contract_address}\``);
      }
    }
    if (jobs.mosaic_vending_minter_e2e) {
      sections.push(formatJobStatus('E2E Tests', jobs.mosaic_vending_minter_e2e.result));
    }
    sections.push('');
  }

  // Full E2E section
  if (jobs.full_e2e) {
    sections.push('**Integration Tests**');
    sections.push(formatJobStatus('Full E2E', jobs.full_e2e.result));
    sections.push('');
  }

  // Footer with run URL
  const runUrl = `https://github.com/${metadata.repository}/actions/runs/${metadata.run_id}`;
  sections.push(`[View Run Details](${runUrl})`);

  return sections.join('\n');
}

async function sendDiscordMessage(message) {
  const webhookUrl = process.env.DISCORD_WEBHOOK;
  if (!webhookUrl) {
    throw new Error('Missing DISCORD_WEBHOOK environment variable');
  }

  const response = await fetch(webhookUrl, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({
      content: message,
    }),
  });

  if (!response.ok) {
    throw new Error(`Failed to send Discord message: ${response.statusText}`);
  }
}

async function main() {
  try {
    const planResults = JSON.parse(process.env.PLAN_RESULTS);
    const message = generateDiscordMessage(planResults);
    await sendDiscordMessage(message);
    console.log('Successfully sent Discord notification');
  } catch (error) {
    console.error('Error sending Discord notification:', error);
    process.exit(1);
  }
}

await main(); 