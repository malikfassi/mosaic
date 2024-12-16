import { getClient, queryAllBalances } from './utils/chain.js';
import { getDeployAddresses } from './utils/deploy.js';
import { createBadgeFiles } from './utils/badges.js';
import { updateGistFiles } from './utils/gist.js';

const EXECUTION_PLAN_GIST_ID = process.env.GIST_ID;
const PROJECT_GIST_ID = process.env.PROJECT_GIST_ID;
const GIST_TOKEN = process.env.GIST_SECRET;

async function updateBalanceBadges(balances) {
    if (!PROJECT_GIST_ID || !GIST_TOKEN) {
        console.log('Skipping gist update - missing PROJECT_GIST_ID or GIST_SECRET');
        return;
    }

    console.log('Debug: Updating balance badges in gist:', PROJECT_GIST_ID);
    const files = createBadgeFiles(balances);
    await updateGistFiles(PROJECT_GIST_ID, GIST_TOKEN, files);
    console.log('Balance badges updated successfully');
}

async function main() {
    try {
        // Get client and addresses
        const client = await getClient();
        const addresses = await getDeployAddresses(EXECUTION_PLAN_GIST_ID, GIST_TOKEN);
        console.log('Found addresses:', addresses);

        // Query all balances
        const balances = await queryAllBalances(client, addresses);

        // Update gist with balance data
        await updateBalanceBadges(balances);
    } catch (error) {
        console.error('Error updating balances:', error);
        process.exit(1);
    }
}

await main(); 