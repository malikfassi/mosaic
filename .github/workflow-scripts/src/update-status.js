import { getOctokit } from '@actions/github';
import { JOBS, COMPONENTS, getAllFileNames, getJobInfo } from './workflow-config.js';

// Map job names to their result environment variables
const JOB_RESULT_MAP = {
  'frontend-ci-lint': 'frontend_lint_result',
  'frontend-ci-test': 'frontend_test_result',
  'frontend-ci-build': 'frontend_build_result',
  
  'mosaic-tile-ci-format': 'mosaic_tile_format_result',
  'mosaic-tile-ci-lint': 'mosaic_tile_lint_result',
  'mosaic-tile-ci-test': 'mosaic_tile_test_result',
  'mosaic-tile-ci-schema': 'mosaic_tile_schema_result',
  'deploy-mosaic-tile': 'mosaic_tile_deploy_result',
  'mosaic-tile-e2e': 'mosaic_tile_e2e_result',
  
  'mosaic-vending-ci-format': 'mosaic_vending_format_result',
  'mosaic-vending-ci-lint': 'mosaic_vending_lint_result',
  'mosaic-vending-ci-test': 'mosaic_vending_test_result',
  'mosaic-vending-ci-schema': 'mosaic_vending_schema_result',
  'deploy-mosaic-vending': 'mosaic_vending_deploy_result',
  'mosaic-vending-e2e': 'mosaic_vending_e2e_result',
  
  'full-e2e': 'full_e2e_result'
};

async function updateGistContent(gist, plan, results) {
  const files = {};

  // Store individual job results
  for (const [jobName, result] of Object.entries(results)) {
    if (result === 'success') {
      const filename = `${jobName}.${plan.components[getJobInfo(jobName).component]}.json`;
      files[filename] = {
        content: JSON.stringify({
          success: true,
          timestamp: new Date().toISOString(),
          run_id: plan.metadata.run_id,  // Store current run ID
          job: jobName,
          workflow_id: plan.metadata.workflow_id,
          component: getJobInfo(jobName).component,
          component_hash: plan.components[getJobInfo(jobName).component]
        }, null, 2)
      };
    }
  }

  await octokit.rest.gists.update({
    gist_id: gist.id,
    files
  });
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
      const [jobName] = filename.split('.');
      const resultKey = JOB_RESULT_MAP[jobName];
      if (!resultKey) {
        console.warn(`No result mapping found for job: ${jobName}`);
        continue;
      }
      const result = results[resultKey];

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
            component: plan.components[jobName],
            componentHash: plan.components[jobName],
            branch: plan.metadata.branch
          }
        };

        // Add deployment info for deploy jobs
        if (jobName.startsWith('deploy-')) {
          content.deployment = {
            network: {
              chain_id: 'elgafar-1',
              rpc_endpoint: 'https://rpc.elgafar-1.stargaze-apis.com:443',
              explorer: 'https://testnet-explorer.publicawesome.dev/stargaze'
            },
            contract: {
              code_id: process.env[`${jobName.toUpperCase()}_CODE_ID`],
              address: process.env[`${jobName.toUpperCase()}_CONTRACT_ADDRESS`],
              name: jobName.replace('deploy-', '')
            }
          };
        }

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