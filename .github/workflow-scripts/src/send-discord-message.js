import { getJobInfo } from './workflow-config.js';

function formatJobResult(result, jobName, plan) {
  if (!result) return '⚪️ Not run';
  
  const jobInfo = plan.jobs[jobName];
  const previousRun = jobInfo?.previous_run;
  
  const status = {
    success: '✅ Success',
    skipped: '⏭️ Skipped',
    failure: '❌ Failed'
  }[result.toLowerCase()] || `⚠️ ${result}`;

  if (previousRun?.run_id) {
    return `${status} ([Previous Run](https://github.com/${plan.metadata.repository}/actions/runs/${previousRun.run_id}))`;
  }

  return status;
}

async function main() {
  const plan = JSON.parse(process.env.EXECUTION_PLAN);
  const results = Object.fromEntries(
    Object.entries(JOB_RESULT_MAP).map(([job, env]) => 
      [job, process.env[env]]
    )
  );

  const message = generateDiscordMessage(plan, results);
  await sendDiscordMessage(message, process.env.DISCORD_WEBHOOK);
} 