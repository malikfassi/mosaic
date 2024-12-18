import fetch from 'node-fetch';
import { COMPONENT_TYPES } from './workflow-config.js';
import {
    generateMetadata,
    generateComponentSection,
    generateSummary
} from './utils/discord.js';

function generateDiscordMessage(planResults) {
    const { metadata, jobs } = planResults;
    const sections = [];

    // Add metadata section
    sections.push(...generateMetadata(metadata));

    // Add component sections
    sections.push(...generateComponentSection(COMPONENT_TYPES.FRONTEND, jobs));
    sections.push(...generateComponentSection(COMPONENT_TYPES.MOSAIC_TILE, jobs));
    
    // Add full E2E section if present
    if (jobs.full_e2e) {
        sections.push('**Integration Tests**');
        sections.push(...generateComponentSection(COMPONENT_TYPES.ALL, jobs));
    }

    // Add summary section
    sections.push(...generateSummary(jobs));
    sections.push('');

    // Add footer with run URL
    const runUrl = `<https://github.com/${metadata.repository}/actions/runs/${metadata.run_id}>`;
    sections.push(`[View Full Run Details](${runUrl})`);

    return sections.join('\n');
}

async function sendDiscordMessage(message) {
    const webhookUrl = process.env.DISCORD_WEBHOOK;
    if (!webhookUrl) {
        throw new Error('Missing DISCORD_WEBHOOK environment variable');
    }

    const response = await fetch(webhookUrl, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({
            content: message,
        }),
    });

    if (!response.ok) {
        throw new Error(`Failed to send Discord message: ${response.statusText}`);
    }
}

async function main() {
    try {
        const planResults = JSON.parse(process.env.PLAN_RESULTS);
        const message = generateDiscordMessage(planResults);
        await sendDiscordMessage(message);
        console.log('Successfully sent Discord notification');
    } catch (error) {
        console.error('Error sending Discord notification:', error);
        process.exit(1);
    }
}

await main(); 