import { getOctokit } from '@actions/github';
import { COMPONENTS, getAllJobs, getAllFileNames } from './workflow-config.js';

async function getStatus(components) {
  try {
    const token = process.env.GIST_SECRET;
    const gistId = process.env.GIST_ID;

    if (!token || !gistId) {
      throw new Error('Missing required environment variables');
    }

    const octokit = getOctokit(token);
    const allJobs = getAllJobs();
    
    // Get gist content once
    const { data: gist } = await octokit.rest.gists.get({ gist_id: gistId }).catch(error => {
      if (error.status === 404) {
        console.warn('No gist found');
        return { data: { files: {} } };
      }
      throw error;
    });

    const allFileNames = getAllFileNames(components);
    
    // Process each file
    const statuses = allFileNames.map((filename) => {
      const file = gist.files[filename];
      const jobName = filename.split('.')[0]; // Get job name from filename
      const jobConfig = allJobs[jobName];
      
      if (!jobConfig) {
        console.warn(`No job config found for ${jobName}`);
        return null;
      }

      let result;
      if (file && file.content) {
        try {
          const content = JSON.parse(file.content);
          result = {
            exists: true,
            success: content.success || false,
            timestamp: content.timestamp || null,
            run_id: content.run_id || null,
            type: jobConfig.type
          };
        } catch (error) {
          console.warn(`Error parsing ${filename}: ${error.message}`);
          result = { 
            exists: false,
            success: false,
            timestamp: null,
            type: jobConfig.type
          };
        }
      } else {
        result = { 
          exists: false, 
          type: jobConfig.type 
        };
      }

      return {
        component: jobConfig.component,
        hash: components[jobConfig.component]?.hash,
        jobs: { [jobName]: result }
      };
    }).filter(Boolean);

    // Merge results by component
    const mergedStatuses = statuses.reduce((acc, status) => {
      if (!acc[status.component]) {
        acc[status.component] = {
          component: status.component,
          hash: status.hash,
          jobs: {}
        };
      }
      Object.assign(acc[status.component].jobs, status.jobs);
      return acc;
    }, {});

    return Object.values(mergedStatuses);
  } catch (error) {
    console.error('Error getting statuses:', error);
    throw error;
  }
}

export { getStatus }; 