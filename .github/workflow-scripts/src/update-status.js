import { getOctokit } from '@actions/github';

async function updateGistFiles(planResults) {
  const gistId = process.env.GIST_ID;
  const gistToken = process.env.GIST_SECRET;

  if (!gistId || !gistToken) {
    throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
  }

  const octokit = getOctokit(gistToken);
  let files = {};

  // use env plan results to update gist
  const planResults = JSON.parse(process.env.PLAN_RESULTS);
  console.log('planResults', planResults);

  for (const jobName in planResults.jobs) {
    const job = planResults.jobs[jobName];
    const filename = job.filename;
    if (job.result === 'success') {
      files[filename] = {
        content: JSON.stringify({
          timestamp: new Date().toISOString(),
          ...planResults.metadata,
          job: {
            name: jobName,
            ...job
          }
        }, null, 2)
      };
    }
  }
  // Process frontend results
  if (planResults.results.frontend) {
    const frontendHash = planResults.components.frontend;
    const frontendResults = planResults.results.frontend;

    // Process each frontend job
    Object.entries(frontendResults).forEach(([jobType, result]) => {
      if (result.result === 'success') {
        const filename = `frontend_ci_${jobType}.${frontendHash}.json`;
       
      }
    });
  }

  // Process contract results
  if (planResults.results.contracts) {
    const contractsHash = planResults.components.contracts;
    const contractResults = planResults.results.contracts;

    // Process each contract job
    Object.entries(contractResults).forEach(([jobType, result]) => {
      if (result.result === 'success') {
        const filename = `${jobType}_cw721.${contractsHash}.json`;
        files[filename] = {
          content: JSON.stringify({
            success: true,
            timestamp: new Date().toISOString(),
            run: {
              id: planResults.metadata.run_id,
              job: `${jobType}_cw721`,
              workflow_id: planResults.metadata.workflow_id,
              commit_sha: planResults.metadata.commit_sha,
              repository: planResults.metadata.repository,
              branch: planResults.metadata.branch
            },
            data: result.data || {}
          }, null, 2)
        };
      }
    });
  }

  // Update gist with all files
  await octokit.rest.gists.update({
    gist_id: gistId,
    files
  });

  console.log('Successfully updated gist files:', Object.keys(files));
}

async function main() {
  try {
    const planResults = JSON.parse(process.env.PLAN_RESULTS);
    await updateGistFiles(planResults);
  } catch (error) {
    console.error('Error updating gist:', error);
    process.exit(1);
  }
}

await main();