import { JOBS, COMPONENTS, getAllJobs } from './workflow-config.js';

function generateExecutionPlan(components) {
  const allJobs = getAllJobs();
  const plan = {
    jobs: {}
  };

  // Add all jobs
  Object.entries(allJobs).forEach(([jobName, jobConfig]) => {
    const component = components[jobConfig.component];
    plan.jobs[jobName] = {
      component: jobConfig.component,
      needs_run: !component?.gist_exists || false
    };
  });

  return plan;
}

export { generateExecutionPlan }; 