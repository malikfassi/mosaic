import { JOB_TYPES, COMPONENT_TYPES } from '../workflow-config.js';

// Format job status with emoji and previous run info
export function formatJobStatus(jobName, jobInfo) {
    const { result, previous_run } = jobInfo;
    const emoji = getStatusEmoji(result);
    let message = `${emoji} ${jobName}: ${result}`;
    
    if (result === 'skipped' && previous_run) {
        const prevRunUrl = `<https://github.com/${previous_run.repository}/actions/runs/${previous_run.run_id}>`;
        message += `\n    ↳ Using results from [previous successful run]${prevRunUrl} (${previous_run.commit_sha.substring(0, 7)})`;
    }
    
    return message;
}

// Get emoji for job status
function getStatusEmoji(result) {
    switch (result) {
        case 'success': return '✅';
        case 'skipped': return '⏭️';
        default: return '❌';
    }
}

// Get jobs for a specific component
export function getJobsByComponent(component) {
    return Object.entries(JOB_TYPES)
        .filter(([_, jobType]) => jobType.startsWith(component))
        .map(([_, jobType]) => jobType);
}

// Get component hash from jobs
export function getComponentHash(jobs, componentJobs) {
    const firstJob = componentJobs.find(jobName => jobs[jobName]);
    if (firstJob && jobs[firstJob]?.component) {
        return jobs[firstJob].component.hash;
    }
    return null;
}

// Format deploy data if available
export function formatDeployData(job) {
    const lines = [];
    if (job.data?.code_id) {
        lines.push(`    ↳ Code ID: \`${job.data.code_id}\``);
    }
    if (job.data?.contract_address) {
        lines.push(`    ↳ Contract: \`${job.data.contract_address}\``);
    }
    return lines;
}

// Generate summary section
export function generateSummary(jobs) {
    const results = Object.values(jobs).map(job => job.result);
    const totalJobs = results.length;
    const successCount = results.filter(r => r === 'success').length;
    const skippedCount = results.filter(r => r === 'skipped').length;
    const failedCount = results.filter(r => r === 'failure').length;

    const lines = ['**Summary**'];
    lines.push(`Total Jobs: ${totalJobs}`);
    if (successCount > 0) lines.push(`✅ Success: ${successCount}`);
    if (skippedCount > 0) lines.push(`⏭️ Skipped: ${skippedCount}`);
    if (failedCount > 0) lines.push(`❌ Failed: ${failedCount}`);

    return lines;
}

// Generate metadata section
export function generateMetadata(metadata) {
    const repoUrl = `<https://github.com/${metadata.repository}>`;
    const runUrl = `<${repoUrl}/actions/runs/${metadata.run_id}>`;
    const commitUrl = `<${repoUrl}/commit/${metadata.commit_sha}>`;
    const branchUrl = `<${repoUrl}/tree/${metadata.branch}>`;

    return [
        `**Workflow Run: ${metadata.workflow_id} [#${metadata.run_number}]${runUrl}**`,
        `Repository: [${metadata.repository}]${repoUrl}`,
        `Branch: [\`${metadata.branch}\`]${branchUrl}`,
        `Commit: [\`${metadata.commit_sha.substring(0, 7)}\`]${commitUrl}`,
        ''
    ];
}

// Generate component section
export function generateComponentSection(componentType, jobs) {
    const lines = [];
    const componentJobs = getJobsByComponent(componentType);
    
    if (componentJobs.some(job => jobs[job])) {
        const hash = getComponentHash(jobs, componentJobs);
        lines.push(`**${formatComponentName(componentType)}** ${hash ? `\`${hash.substring(0, 8)}\`` : ''}`);
        
        // Add job statuses
        componentJobs.forEach(jobName => {
            if (jobs[jobName]) {
                lines.push(formatJobStatus(jobName, jobs[jobName]));
                // Add deploy data if available
                if (jobName.includes('deploy') && jobs[jobName].data) {
                    lines.push(...formatDeployData(jobs[jobName]));
                }
            }
        });
        
        lines.push('');
    }
    
    return lines;
}

// Format component name for display
function formatComponentName(componentType) {
    return componentType
        .split('_')
        .map(word => word.charAt(0).toUpperCase() + word.slice(1))
        .join(' ');
} 