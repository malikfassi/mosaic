import { writeFile } from 'fs/promises';
import { tryParseJson } from './workflow-config.js';

// Parse job outputs from environment variables
function getJobResults(jobName) {
    const resultKey = `${jobName.toUpperCase()}_RESULT`;
    const outputsKey = `${jobName.toUpperCase()}_OUTPUTS`;
    
    const result = process.env[resultKey] || '';
    let data = null;

    try {
        const outputsStr = process.env[outputsKey];
        if (outputsStr) {
            const outputs = tryParseJson(outputsStr.trim());
            if (!outputs) {
                console.warn(`Failed to parse outputs for ${jobName}`);
                return { result, data };
            }
            
            // Handle both formats:
            // 1. JSON string with {code_id, contract_address}
            // 2. Full JSON object with timestamp, run_id etc
            if (outputs.code_id || outputs.contract_address) {
                data = outputs;
            } else if (outputs.job?.data) {
                data = outputs.job.data;
            }
        }
    } catch (err) {
        console.error(`Failed to parse outputs for ${jobName}:`, err);
    }

    return { result, data };
}

// Generate plan results from execution plan and job results
function generatePlanResults() {
    const executionPlan = tryParseJson(process.env.EXECUTION_PLAN);
    if (!executionPlan) {
        throw new Error('Failed to parse execution plan');
    }

    const results = {
        jobs: {},
        metadata: executionPlan.metadata
    };

    // Process each job
    for (const [jobName, jobInfo] of Object.entries(executionPlan.jobs)) {
        const { result, data } = getJobResults(jobName);
        
        results.jobs[jobName] = {
            ...jobInfo,
            result,
            data
        };
    }

    return results;
}

async function main() {
    try {
        const planResults = generatePlanResults();

        // Save plan results
        await writeFile('plan-results.json', JSON.stringify(planResults, null, 2));
        console.log('Successfully generated plan results:', planResults);

    } catch (error) {
        console.error('Error generating plan results:', error);
        process.exit(1);
    }
}

await main(); 