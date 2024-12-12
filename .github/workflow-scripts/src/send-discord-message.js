import fetch from 'node-fetch';
import { JOBS } from './workflow-config.js';

function formatJobStatus(jobName, jobInfo) {
  const { result, previous_run } = jobInfo;
  const emoji = result === 'success' ? '✅' : result === 'skipped' ? '⏭️' : '❌';
  let message = `${emoji} ${jobName}: ${result}`;
  
  if (result === 'skipped' && previous_run) {
    const prevRunUrl = `https://github.com/${previous_run.repository}/actions/runs/${previous_run.run_id}`;
    message += `\n    ↳ Using results from [previous successful run](${prevRunUrl}) (${previous_run.commit_sha.substring(0, 7)})`;
  }
  
  return message;
}

function getJobsByComponent(component) {
  return Object.entries(JOBS)
    .filter(([_, config]) => config.component.name === component)
    .map(([jobName]) => jobName);
}

function getComponentHash(jobs, componentJobs) {
  // Get the first job for this component
  const firstJob = componentJobs.find(jobName => jobs[jobName]);
  if (firstJob && jobs[firstJob] && jobs[firstJob].component) {
    return jobs[firstJob].component.hash;
  }
  return null;
}

function generateDiscordMessage(planResults) {
  const { metadata, jobs } = planResults;
  
  // Build message sections
  const sections = [];

  // Header with metadata
  const repoUrl = `https://github.com/${metadata.repository}`;
  const runUrl = `${repoUrl}/actions/runs/${metadata.run_id}`;
  const commitUrl = `${repoUrl}/commit/${metadata.commit_sha}`;

  sections.push(`**Workflow Run: ${metadata.workflow_id} [#${metadata.run_number}](${runUrl})**`);
  sections.push(`Repository: [${metadata.repository}](${repoUrl})`);
  sections.push(`Branch: [\`${metadata.branch}\`](${repoUrl}/tree/${metadata.branch})`);
  sections.push(`Commit: [\`${metadata.commit_sha.substring(0, 7)}\`](${commitUrl})`);
  sections.push('');

  // Frontend section
  const frontendJobs = getJobsByComponent('frontend');
  if (frontendJobs.some(job => jobs[job])) {
    const hash = getComponentHash(jobs, frontendJobs);
    sections.push(`**Frontend** ${hash ? `\`${hash.substring(0, 8)}\`` : ''}`);
    if (jobs.frontend_lint) {
      sections.push(formatJobStatus('Lint', jobs.frontend_lint));
    }
    if (jobs.frontend_test) {
      sections.push(formatJobStatus('Test', jobs.frontend_test));
    }
    if (jobs.frontend_build) {
      sections.push(formatJobStatus('Build', jobs.frontend_build));
    }
    sections.push('');
  }

  // Mosaic Tile section
  const tileJobs = getJobsByComponent('mosaic_tile');
  if (tileJobs.some(job => jobs[job])) {
    const hash = getComponentHash(jobs, tileJobs);
    sections.push(`**Mosaic Tile** ${hash ? `\`${hash.substring(0, 8)}\`` : ''}`);
    if (jobs.mosaic_tile_nft_fmt) {
      sections.push(formatJobStatus('Format', jobs.mosaic_tile_nft_fmt));
    }
    if (jobs.mosaic_tile_nft_clippy) {
      sections.push(formatJobStatus('Lint', jobs.mosaic_tile_nft_clippy));
    }
    if (jobs.mosaic_tile_nft_test) {
      sections.push(formatJobStatus('Test', jobs.mosaic_tile_nft_test));
    }
    if (jobs.mosaic_tile_nft_compile) {
      sections.push(formatJobStatus('Compile', jobs.mosaic_tile_nft_compile));
    }
    if (jobs.mosaic_tile_nft_deploy) {
      sections.push(formatJobStatus('Deploy', jobs.mosaic_tile_nft_deploy));
      if (jobs.mosaic_tile_nft_deploy.data && jobs.mosaic_tile_nft_deploy.data.code_id) {
        sections.push(`    ↳ Code ID: \`${jobs.mosaic_tile_nft_deploy.data.code_id}\``);
      }
      if (jobs.mosaic_tile_nft_deploy.data && jobs.mosaic_tile_nft_deploy.data.contract_address) {
        sections.push(`    ↳ Contract: \`${jobs.mosaic_tile_nft_deploy.data.contract_address}\``);
      }
    }
    if (jobs.mosaic_tile_nft_e2e) {
      sections.push(formatJobStatus('E2E Tests', jobs.mosaic_tile_nft_e2e));
    }
    sections.push('');
  }

  // Mosaic Vending section
  const vendingJobs = getJobsByComponent('mosaic_vending');
  if (vendingJobs.some(job => jobs[job])) {
    const hash = getComponentHash(jobs, vendingJobs);
    sections.push(`**Mosaic Vending** ${hash ? `\`${hash.substring(0, 8)}\`` : ''}`);
    if (jobs.mosaic_vending_minter_fmt) {
      sections.push(formatJobStatus('Format', jobs.mosaic_vending_minter_fmt));
    }
    if (jobs.mosaic_vending_minter_clippy) {
      sections.push(formatJobStatus('Lint', jobs.mosaic_vending_minter_clippy));
    }
    if (jobs.mosaic_vending_minter_test) {
      sections.push(formatJobStatus('Test', jobs.mosaic_vending_minter_test));
    }
    if (jobs.mosaic_vending_minter_compile) {
      sections.push(formatJobStatus('Compile', jobs.mosaic_vending_minter_compile));
    }
    if (jobs.mosaic_vending_minter_deploy) {
      sections.push(formatJobStatus('Deploy', jobs.mosaic_vending_minter_deploy));
      if (jobs.mosaic_vending_minter_deploy.data && jobs.mosaic_vending_minter_deploy.data.code_id) {
        sections.push(`    ↳ Code ID: \`${jobs.mosaic_vending_minter_deploy.data.code_id}\``);
      }
      if (jobs.mosaic_vending_minter_deploy.data && jobs.mosaic_vending_minter_deploy.data.contract_address) {
        sections.push(`    ↳ Contract: \`${jobs.mosaic_vending_minter_deploy.data.contract_address}\``);
      }
    }
    if (jobs.mosaic_vending_minter_e2e) {
      sections.push(formatJobStatus('E2E Tests', jobs.mosaic_vending_minter_e2e));
    }
    sections.push('');
  }

  // Full E2E section
  if (jobs.full_e2e) {
    const hash = getComponentHash(jobs, ['full_e2e']);
    sections.push(`**Integration Tests** ${hash ? `\`${hash.substring(0, 8)}\`` : ''}`);
    sections.push(formatJobStatus('Full E2E', jobs.full_e2e));
    sections.push('');
  }

  // Summary of results
  const results = Object.values(jobs).map(job => job.result);
  const totalJobs = results.length;
  const successCount = results.filter(r => r === 'success').length;
  const skippedCount = results.filter(r => r === 'skipped').length;
  const failedCount = results.filter(r => r === 'failure').length;

  sections.push('**Summary**');
  sections.push(`Total Jobs: ${totalJobs}`);
  if (successCount > 0) sections.push(`✅ Success: ${successCount}`);
  if (skippedCount > 0) sections.push(`⏭️ Skipped: ${skippedCount}`);
  if (failedCount > 0) sections.push(`❌ Failed: ${failedCount}`);
  sections.push('');

  // Footer with run URL
  sections.push(`[View Full Run Details](${runUrl})`);

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