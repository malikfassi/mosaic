import { getOctokit } from '@actions/github';
import { tryParseJson } from '../workflow-config.js';

export async function getGistFiles(gistId, token) {
    if (!gistId || !token) {
        throw new Error('Missing required parameters: gistId or token');
    }

    console.log('Debug: Fetching from gist:', gistId);
    const octokit = getOctokit(token);
    const { data: gist } = await octokit.rest.gists
        .get({ gist_id: gistId })
        .catch((error) => {
            if (error.status === 404) {
                console.warn("No gist found");
                return { data: { files: {} } };
            }
            throw error;
        });

    return gist.files;
}

export async function updateGistFiles(gistId, token, files) {
    if (!gistId || !token) {
        throw new Error('Missing required parameters: gistId or token');
    }

    console.log('Debug: Updating gist:', gistId);
    const octokit = getOctokit(token);
    await octokit.rest.gists.update({
        gist_id: gistId,
        files
    });
}

export function getPreviousRun(gistFiles, filename) {
    const gistFile = gistFiles[filename];
    if (!gistFile) return null;
    
    try {
        return tryParseJson(gistFile.content);
    } catch (error) {
        console.warn(`Debug: Could not parse previous run from ${filename}:`, error);
        return null;
    }
}

export function getLatestRun(gistFiles, pattern) {
    let latestRun = null;
    let latestTimestamp = 0;

    for (const [filename, file] of Object.entries(gistFiles)) {
        if (filename.includes(pattern)) {
            try {
                const run = getPreviousRun(gistFiles, filename);
                if (run && run.job?.data && run.timestamp) {
                    const timestamp = new Date(run.timestamp).getTime();
                    if (timestamp > latestTimestamp) {
                        latestTimestamp = timestamp;
                        latestRun = run;
                        console.log('Debug: Found newer run from:', run.timestamp);
                    }
                }
            } catch (error) {
                console.warn(`Debug: Failed to parse file ${filename}:`, error);
            }
        }
    }

    return latestRun;
} 