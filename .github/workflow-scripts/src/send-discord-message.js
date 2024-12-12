import fetch from 'node-fetch';

function formatJobStatus(jobName, result) {
  const emoji = result === 'success' ? '✅' : result === 'skipped' ? '⏭️' : '❌';
  return `${emoji} ${jobName}: ${result}`;
}

function generateDiscordMessage(planResults) {
  const { metadata, results } = planResults;
  
  // Build message sections
  const sections = [];

  // Header
  sections.push(`**Workflow Run: ${metadata.workflow_id}**`);
  sections.push(`Branch: \`${metadata.branch}\``);
  sections.push(`Commit: \`${metadata.commit_sha.substring(0, 7)}\``);
  sections.push('');

  // Frontend section
  if (results['frontend-ci']) {
    sections.push('**Frontend**');
    sections.push(formatJobStatus('Lint', results['frontend-ci'].lint.result));
    sections.push(formatJobStatus('Test', results['frontend-ci'].test.result));
    sections.push(formatJobStatus('Build', results['frontend-ci'].build.result));
    sections.push('');
  }

  // Mosaic Tile section
  if (results['mosaic-tile']) {
    sections.push('**Mosaic Tile**');
    if (results['mosaic-tile'].ci) {
      sections.push(formatJobStatus('Format', results['mosaic-tile'].ci.format.result));
      sections.push(formatJobStatus('Lint', results['mosaic-tile'].ci.lint.result));
      sections.push(formatJobStatus('Test', results['mosaic-tile'].ci.test.result));
      sections.push(formatJobStatus('Schema', results['mosaic-tile'].ci.schema.result));
    }
    if (results['mosaic-tile'].deploy) {
      sections.push(formatJobStatus('Deploy', results['mosaic-tile'].deploy.result));
      if (results['mosaic-tile'].deploy.outputs) {
        sections.push(`Code ID: \`${results['mosaic-tile'].deploy.outputs.code_id}\``);
        sections.push(`Contract: \`${results['mosaic-tile'].deploy.outputs.contract_address}\``);
      }
    }
    if (results['mosaic-tile'].e2e) {
      sections.push(formatJobStatus('E2E Tests', results['mosaic-tile'].e2e.result));
    }
    sections.push('');
  }

  // Mosaic Vending section
  if (results['mosaic-vending']) {
    sections.push('**Mosaic Vending**');
    if (results['mosaic-vending'].ci) {
      sections.push(formatJobStatus('Format', results['mosaic-vending'].ci.format.result));
      sections.push(formatJobStatus('Lint', results['mosaic-vending'].ci.lint.result));
      sections.push(formatJobStatus('Test', results['mosaic-vending'].ci.test.result));
      sections.push(formatJobStatus('Schema', results['mosaic-vending'].ci.schema.result));
    }
    if (results['mosaic-vending'].deploy) {
      sections.push(formatJobStatus('Deploy', results['mosaic-vending'].deploy.result));
      if (results['mosaic-vending'].deploy.outputs) {
        sections.push(`Code ID: \`${results['mosaic-vending'].deploy.outputs.code_id}\``);
        sections.push(`Contract: \`${results['mosaic-vending'].deploy.outputs.contract_address}\``);
      }
    }
    if (results['mosaic-vending'].e2e) {
      sections.push(formatJobStatus('E2E Tests', results['mosaic-vending'].e2e.result));
    }
    sections.push('');
  }

  // Full E2E section
  if (results['full-e2e']) {
    sections.push('**Integration Tests**');
    sections.push(formatJobStatus('Full E2E', results['full-e2e'].result));
    sections.push('');
  }

  // Footer
  sections.push(`Run completed at: ${new Date(metadata.completed_at).toLocaleString()}`);

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