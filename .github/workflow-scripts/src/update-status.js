import { getOctokit } from '@actions/github';

async function updateStatus() {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;
    const jobName = process.env.JOB_NAME;
    const success = process.env.JOB_SUCCESS === 'true';
    const executionPlan = JSON.parse(process.env.EXECUTION_PLAN || '{}');

    if (!token || !gistId || !jobName || !executionPlan) {
      throw new Error('Missing required environment variables');
    }

    // Only update successful jobs
    if (!success) {
      console.log(`Job ${jobName} was not successful, skipping status update`);
      return;
    }

    const octokit = getOctokit(token);
    const jobInfo = executionPlan.jobs[jobName];

    if (!jobInfo) {
      throw new Error(`Job ${jobName} not found in execution plan`);
    }

    // Create status file content - match the format we expect when reading
    const content = JSON.stringify({
      success: true,
      timestamp: new Date().toISOString(),
      run: {
        id: jobInfo.id,
        job: jobName,
        componentName: jobInfo.componentName,
        componentHash: jobInfo.componentHash,
        workflowId: jobInfo.workflowId,
        commitId: jobInfo.commitId,
        results: jobInfo.results || {}
      }
    }, null, 2);

    // Update gist
    const filename = jobName.endsWith('-e2e') 
      ? `${jobName}.${jobInfo.commitId}.json` // E2E jobs use commit hash
      : `${jobName}.${jobInfo.componentHash}.json`; // Component jobs use component hash

    await octokit.rest.gists.update({
      gist_id: gistId,
      files: {
        [filename]: {
          content
        }
      }
    });

    console.log(`Updated status for ${jobName}`);
  } catch (error) {
    console.error('Error updating status:', error);
    throw error;
  }
}

export { updateStatus }; 