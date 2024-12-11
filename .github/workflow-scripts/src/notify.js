import { writeFile } from 'fs/promises';
import { JOBS, COMPONENTS } from './workflow-config.js';

function formatJobResult(result) {
  if (!result) return '⚪️ Not run';
  switch (result.toLowerCase()) {
    case 'success':
      return '✅ Success';
    case 'skipped':
      return '⏭️ Skipped';
    case 'failure':
      return '❌ Failed';
    default:
      return `⚠️ ${result}`;
  }
}

function generateComponentStatus(name, jobs, results) {
  const lines = [];
  lines.push(`**${name}**`);
  
  for (const jobName of jobs) {
    const result = results[jobName.toLowerCase().replace(/-/g, '_')];
    lines.push(`- ${jobName}: ${formatJobResult(result)}`);
  }
  
  return lines.join('\n');
}

async function main() {
  try {
    // Get inputs from environment
    const plan = JSON.parse(process.env.EXECUTION_PLAN);
    const results = {
      frontend_lint_result: process.env.FRONTEND_LINT_RESULT,
      frontend_test_result: process.env.FRONTEND_TEST_RESULT,
      frontend_build_result: process.env.FRONTEND_BUILD_RESULT,
      
      mosaic_tile_format_result: process.env.MOSAIC_TILE_FORMAT_RESULT,
      mosaic_tile_lint_result: process.env.MOSAIC_TILE_LINT_RESULT,
      mosaic_tile_test_result: process.env.MOSAIC_TILE_TEST_RESULT,
      mosaic_tile_schema_result: process.env.MOSAIC_TILE_SCHEMA_RESULT,
      mosaic_tile_deploy_result: process.env.MOSAIC_TILE_DEPLOY_RESULT,
      mosaic_tile_e2e_result: process.env.MOSAIC_TILE_E2E_RESULT,
      
      mosaic_vending_format_result: process.env.MOSAIC_VENDING_FORMAT_RESULT,
      mosaic_vending_lint_result: process.env.MOSAIC_VENDING_LINT_RESULT,
      mosaic_vending_test_result: process.env.MOSAIC_VENDING_TEST_RESULT,
      mosaic_vending_schema_result: process.env.MOSAIC_VENDING_SCHEMA_RESULT,
      mosaic_vending_deploy_result: process.env.MOSAIC_VENDING_DEPLOY_RESULT,
      mosaic_vending_e2e_result: process.env.MOSAIC_VENDING_E2E_RESULT,
      
      full_e2e_result: process.env.FULL_E2E_RESULT
    };

    // Generate status sections
    const sections = [];
    
    // Header
    sections.push('🔄 **Mosaic CI/CD Status**\n');
    
    // Frontend status
    if (COMPONENTS.frontend) {
      sections.push(generateComponentStatus('Frontend', COMPONENTS.frontend.jobs, results));
    }
    
    // Mosaic Tile status
    if (COMPONENTS.mosaic_tile) {
      sections.push(generateComponentStatus('Mosaic Tile', COMPONENTS.mosaic_tile.jobs, results));
    }
    
    // Mosaic Vending status
    if (COMPONENTS.mosaic_vending) {
      sections.push(generateComponentStatus('Mosaic Vending', COMPONENTS.mosaic_vending.jobs, results));
    }
    
    // Integration status
    sections.push('**Integration**');
    sections.push(`- Full E2E: ${formatJobResult(results.full_e2e_result)}`);
    
    // Add run link
    sections.push(`\n[View run](${plan.metadata.repository}/actions/runs/${plan.metadata.run_id})`);

    // Join all sections and write to file
    const message = sections.join('\n\n');
    await writeFile('discord_message.txt', message);
    
    console.log('Successfully generated Discord message');
  } catch (error) {
    console.error('Error in notify:', error);
    process.exit(1);
  }
}

await main();