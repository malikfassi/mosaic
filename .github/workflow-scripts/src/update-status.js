import { getOctokit } from '@actions/github';

async function updateGistFiles(planResults) {
  const gistId = process.env.GIST_ID;
  const gistToken = process.env.GIST_SECRET;

  if (!gistId || !gistToken) {
    throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
  }

  const octokit = getOctokit(gistToken);
  let files = {};

  for (const jobName in planResults.jobs) {
    const job = planResults.jobs[jobName];
    const filename = job.filename;
    if (job.result === 'success') {
      const jobRecord = {
        timestamp: new Date().toISOString(),
        run_id: planResults.metadata.run_id,
        run_number: planResults.metadata.run_number,
        commit_sha: planResults.metadata.commit_sha,
        workflow_id: planResults.metadata.workflow_id,
        branch: planResults.metadata.branch,
        repository: planResults.metadata.repository,
        job: {
          name: jobName,
          component: {
            name: job.component.name,
            hash: job.component.hash
          },
          result: job.result,
          data: job.data || {}
        }
      };

      files[filename] = {
        content: JSON.stringify(jobRecord, null, 2)
      };
    }
  }

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