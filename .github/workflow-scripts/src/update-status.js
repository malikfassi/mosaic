import { updateGistFiles } from './utils/gist.js';
import { createJobRecord } from './utils/jobs.js';

const GIST_ID = process.env.GIST_ID;
const GIST_TOKEN = process.env.GIST_SECRET;

async function updateJobStatuses(planResults) {
    if (!GIST_ID || !GIST_TOKEN) {
        throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
    }

    const files = {};
    for (const [jobName, job] of Object.entries(planResults.jobs)) {
        if (!job.result) {
            console.warn(`Warning: Job ${jobName} has no result`);
            continue;
        }

        // Only save successful jobs
        if (job.result === 'success') {
            console.log(`Saving successful job ${jobName}`);
            files[job.filename] = {
                content: JSON.stringify(createJobRecord(jobName, job, planResults.metadata), null, 2)
            };
        } else {
            console.log(`Not saving job ${jobName} with result: ${job.result}`);
        }
    }

    if (Object.keys(files).length > 0) {
        await updateGistFiles(GIST_ID, GIST_TOKEN, files);
        console.log('Successfully updated gist files:', Object.keys(files));
    } else {
        console.log('No successful jobs to update');
    }
}

async function main() {
    try {
        const planResults = JSON.parse(process.env.PLAN_RESULTS);
        await updateJobStatuses(planResults);
    } catch (error) {
        console.error('Error updating status:', error);
        process.exit(1);
    }
}

await main();