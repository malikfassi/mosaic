import { getOctokit } from '@actions/github';
import { COMPONENTS, getComponentJobs } from './workflow-config.js';
import { getAllFileNames } from './workflow-config.js';

async function getStatus(components) {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;

    if (!token || !gistId) {
      throw new Error('Missing required environment variables');
    }

    const octokit = getOctokit(token);
    
    // Get gist content once
    const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId }).catch(error => {
      if (error.status === 404) {
        console.warn('No gist found');
        return { data: { files: {} } };
      }
      throw error;
    });

    const allFileNames = getAllFileNames(components)
    // Process each component
    const statuses = allFileNames.map((filename) => {
        const results = {};

        const file = gist.files[filename];
        
        if (file && file.content) {
          try {
            const content = JSON.parse(file.content);
            results[jobName] = {
              exists: true,
              success: content.success || false,
              timestamp: content.timestamp || null,
              run_id: content.run_id || null,
              type: jobConfig.type
            };
          } catch (error) {
            console.warn(`Error parsing ${filename}: ${error.message}`);
            results[jobName] = { 
                exists: false,
                success: false,
                timestamp: null,
                type: jobConfig.type
            };
          }
        } else {
          results[jobName] = { exists: false, type: jobConfig.type };
        }
        return {
            component,
            hash: data.hash,
            jobs: results
        };
    });
    return statuses;
  }
   catch (error) {
    console.error('Error getting statuses:', error);
    throw error;
  }
}

export { getStatus }; 