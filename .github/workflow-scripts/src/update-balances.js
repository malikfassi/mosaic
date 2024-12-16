import { getGistFiles, getLatestRun, updateGistFiles } from './utils/gist.js';
import { getClient, queryBalance, convertUstarsToStars } from './utils/chain.js';
import { createBadgeFiles } from './utils/badges.js';

const EXECUTION_PLAN_GIST_ID = process.env.GIST_ID;
const PROJECT_GIST_ID = process.env.PROJECT_GIST_ID;
const GIST_TOKEN = process.env.GIST_SECRET;

async function getAddressesFromExecutionPlan() {
    const files = await getGistFiles(EXECUTION_PLAN_GIST_ID, GIST_TOKEN);
    const addresses = {
        deployer: null,
        minter: null,
        owner: null,
        user: null
    };

    console.log('Debug: Looking for mosaic_tile_nft_deploy job output');
    
    // Get the latest successful deploy job
    const latestRun = getLatestRun(files, 'mosaic_tile_nft_deploy');

    if (latestRun) {
        console.log('Debug: Using deploy data from run:', latestRun.timestamp);
        const data = latestRun.job.data;
        addresses.deployer = data.deployer_address;
        addresses.minter = data.minter_address;
        addresses.owner = data.owner_address;
        addresses.user = data.user_address;
        console.log('Debug: Extracted addresses:', addresses);
    } else {
        console.log('Debug: No successful deploy job found');
    }

    // Validate that we have all addresses
    for (const [role, address] of Object.entries(addresses)) {
        if (!address) {
            console.log('Debug: Missing address for role:', role);
            console.log('Debug: Current addresses:', addresses);
            throw new Error(`Missing ${role} address in execution plan data`);
        }
    }

    return addresses;
}

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
        const client = await getClient();
        
        // Get addresses from execution plan
        const addresses = await getAddressesFromExecutionPlan();
        console.log('Found addresses:', addresses);

        // Query balances
        const balances = await Promise.all(
            Object.entries(addresses).map(async ([role, address]) => {
                const balance = await queryBalance(client, address);
                return {
                    role,
                    ...balance,
                    stars: convertUstarsToStars(balance.balance)
                };
            })
        );

        // Update gist with balance data
        await updateBalanceBadges(balances);
    } catch (error) {
        console.error('Error updating balances:', error);
        process.exit(1);
    }
}

await main(); 