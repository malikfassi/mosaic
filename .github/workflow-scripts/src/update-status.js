import { getOctokit } from '@actions/github';
import { JOBS, COMPONENTS, getAllFileNames } from './workflow-config.js';

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

    const gistId = process.env.GIST_ID;
    const gistToken = process.env.GIST_SECRET;

    if (!gistId || !gistToken) {
      throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
    }

    // Initialize Octokit
    const octokit = getOctokit(gistToken);

    // Get all filenames based on component hashes
    const filenames = getAllFileNames(plan.components, plan.metadata.commit_sha);

    // Prepare gist files update
    const files = {};
    for (const filename of filenames) {
      const [jobName, hash] = filename.split('.');
      const result = results[`${jobName.toLowerCase().replace(/-/g, '_')}_result`];

      // Only update files for successful jobs
      if (result === 'success') {
        const content = {
          success: true,
          timestamp: new Date().toISOString(),
          run: {
            id: plan.metadata.run_id,
            job: jobName,
            workflow_id: plan.metadata.workflow_id,
            commit_sha: plan.metadata.commit_sha,
            repository: plan.metadata.repository,
            component_hash: hash,
            branch: plan.metadata.branch
          }
        };

        files[filename] = {
          content: JSON.stringify(content, null, 2)
        };
      }
    }

    // Update gist
    await octokit.rest.gists.update({
      gist_id: gistId,
      files
    });

    console.log('Successfully updated gist with job results:');
    console.log('Files updated:', Object.keys(files));
  } catch (error) {
    console.error('Error updating gist:', error);
    process.exit(1);
  }
}

await main();