// Job record creation and validation
export function createJobRecord(jobName, job, metadata) {
    return {
        timestamp: new Date().toISOString(),
        run_id: metadata.run_id,
        run_number: metadata.run_number,
        commit_sha: metadata.commit_sha,
        workflow_id: metadata.workflow_id,
        branch: metadata.branch,
        repository: metadata.repository,
        job: {
            name: jobName,
            component: {
                name: job.component.name,
                hash: job.component.hash
            },
            result: job.result,
            data: job.data || {}
        }
    };
}

export function updateJobWithHash(job, componentHash) {
    return {
        ...job,
        component: {
            name: job.component.name,
            hash: componentHash
        }
    };
}

export function getJobFilename(jobName, componentHash) {
    return `${jobName}.${componentHash}.json`;
}

export function validateJob(job) {
    if (!job.component?.name) {
        throw new Error('Job must have a component with a name');
    }
    return job;
} 