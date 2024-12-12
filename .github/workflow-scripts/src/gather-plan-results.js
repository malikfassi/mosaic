import { writeFile } from 'fs/promises';
import { tryParseJson } from './workflow-config.js';

function getJobResults() {
  const executionPlan = tryParseJson(process.env.EXECUTION_PLAN);

  const jobNames = Object.keys(executionPlan.jobs);
  jobNames.forEach((jobName) => {
    executionPlan.jobs[jobName].result = process.env[`${jobName.toUpperCase()}_RESULT`];
    executionPlan.jobs[jobName].data = tryParseJson(process.env[`${jobName.toUpperCase()}_OUTPUTS`]);
  });
  return executionPlan;
}

async function main() {
  try {
    const plan_results = getJobResults();

    // Save plan results
    await writeFile('plan-results.json', JSON.stringify(plan_results, null, 2));
    console.log('Successfully generated plan results :', plan_results);

  } catch (error) {
    console.error('Error generating plan results:', error);
    process.exit(1);
  }
}

await main(); 