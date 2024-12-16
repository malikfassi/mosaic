import { writeFile } from "fs/promises";
import { JOBS, COMPONENTS } from "./workflow-config.js";
import { getGistFiles, getPreviousRun } from "./utils/gist.js";
import { generateComponentHashes } from "./utils/component.js";
import { updateJobWithHash, getJobFilename } from "./utils/jobs.js";

async function generateExecutionPlan() {
    const gistId = process.env.GIST_ID;
    const token = process.env.GIST_SECRET;
    const commitSha = process.env.GITHUB_SHA;

    if (!gistId || !token || !commitSha) {
        throw new Error("Missing required environment variables");
    }

    const gistFiles = await getGistFiles(gistId, token);
    const componentHashes = generateComponentHashes(COMPONENTS);
    console.log("Component hashes:", componentHashes);

    // Calculate component hashes and check for previous successful runs
    const updatedJobs = {};
    Object.entries(JOBS).forEach(([jobName, job]) => {
        console.log("Processing job:", jobName);
        console.log("Job component name:", job.component.name);
        console.log("Component hash:", componentHashes[job.component.name]);
        
        // Update job with component hash
        const updatedJob = updateJobWithHash(job, componentHashes[job.component.name]);
        
        // Set filename for this job's results
        const filename = getJobFilename(jobName, componentHashes[job.component.name]);
        updatedJob.filename = filename;

        // Check for previous successful run with same component hash
        updatedJob.previous_run = getPreviousRun(gistFiles, filename);
        
        updatedJobs[jobName] = updatedJob;
    });

    // Generate execution plan
    const plan = {
        jobs: updatedJobs,
        metadata: {
            commit_sha: process.env.GITHUB_SHA,
            workflow_id: process.env.GITHUB_WORKFLOW,
            branch: process.env.GITHUB_REF_NAME,
            run_id: process.env.GITHUB_RUN_ID,
            run_number: process.env.GITHUB_RUN_NUMBER,
            repository: process.env.GITHUB_REPOSITORY,
        },
    };

    const planFile = "execution-plan.json";
    await writeFile(planFile, JSON.stringify(plan, null, 2));
    console.log("Generated execution plan:", JSON.stringify(plan));

    return plan;
}

async function main() {
    try {
        await generateExecutionPlan();
    } catch (error) {
        console.error("Error generating execution plan:", error);
        process.exit(1);
    }
}

await main();
