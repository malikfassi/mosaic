import { writeFile } from 'fs/promises';
import { tryParseJson } from './workflow-config.js';

function getJobResults(jobName) {
  const resultKey = `${jobName.toUpperCase()}_RESULT`;
  const outputsKey = `${jobName.toUpperCase()}_OUTPUTS`;
  
  const result = process.env[resultKey] || '';
  let data = null;

  try {
    const outputsStr = process.env[outputsKey];
    if (outputsStr) {
      // Handle both formats:
      // 1. JSON string with {code_id, contract_address}
      // 2. Full JSON object with timestamp, run_id etc
      const outputs = JSON.parse(outputsStr.trim());
      
      // If it's a simple object with code_id/contract_address
      if (outputs.code_id || outputs.contract_address) {
        data = outputs;
      } 
      // If it's a full job output object
      else if (outputs.job && outputs.job.data) {
        data = outputs.job.data;
      }
    }
  } catch (err) {
    console.error(`Failed to parse outputs for ${jobName}:`, err);
  }

  return {
    result,
    data
  };
}

function generatePlanResults() {
  const executionPlan = JSON.parse(process.env.EXECUTION_PLAN);
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
    const plan_results = generatePlanResults();

    // Save plan results
    await writeFile('plan-results.json', JSON.stringify(plan_results, null, 2));
    console.log('Successfully generated plan results :', plan_results);

  } catch (error) {
    console.error('Error generating plan results:', error);
    process.exit(1);
  }
}

await main(); 