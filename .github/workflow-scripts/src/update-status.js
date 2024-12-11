import { getOctokit } from '@actions/github';

// Update status for a specific job
async function updateJobStatus(octokit, gistId, jobName, componentHash, success) {
  try {
    // Create or update the file
    const filename = `${jobName}.${componentHash}.json`;
    const content = JSON.stringify({
      job: jobName,
      hash: componentHash,
      success,
      timestamp: new Date().toISOString(),
      run_id: process.env.GITHUB_RUN_ID,
      run_number: process.env.GITHUB_RUN_NUMBER,
      workflow: process.env.GITHUB_WORKFLOW,
      repository: process.env.GITHUB_REPOSITORY,
      ref: process.env.GITHUB_REF
    }, null, 2);

    // Update gist
    await octokit.rest.gists.update({
      gist_id: gistId,
      files: {
        [filename]: {
          content
        }
      }
    });

    console.log(`Updated status for ${jobName} (${componentHash}): ${success}`);
    return true;
  } catch (error) {
    if (error.status === 404) {
      console.warn('Gist not found, creating new one...');
      try {
        await octokit.rest.gists.create({
          public: false,
          description: `Job status for ${jobName}`,
          files: {
            [filename]: {
              content
            }
          }
        });
        return true;
      } catch (createError) {
        console.error(`Error creating gist: ${createError.message}`);
        throw createError;
      }
    }
    console.error(`Error updating status for ${jobName}: ${error.message}`);
    throw error;
  }
}

// Main function to update job status
async function updateStatus() {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;
    const jobName = process.env.JOB_NAME;
    const componentHash = process.env.COMPONENT_HASH;
    const success = process.env.JOB_SUCCESS === 'true';

    if (!token || !gistId || !jobName || !componentHash) {
      throw new Error('Missing required environment variables');
    }

    const octokit = getOctokit(token);
    return await updateJobStatus(octokit, gistId, jobName, componentHash, success);
  } catch (error) {
    console.error('Error updating status:', error);
    throw error;
  }
}

// Run if called directly
if (require.main === module) {
  updateStatus()
    .then(result => {
      if (!result) process.exit(1);
    })
    .catch(error => {
      console.error(error);
      process.exit(1);
    });
}

export { updateStatus, updateJobStatus }; 