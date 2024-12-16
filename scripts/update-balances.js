import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { fileURLToPath } from 'url';
import path from 'path';
import fetch from 'node-fetch';

const RPC_ENDPOINT = process.env.STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com:443';
const EXECUTION_PLAN_GIST_ID = process.env.GIST_ID;
const PROJECT_GIST_ID = process.env.PROJECT_GIST_ID || 'c67eb85b7002c9e7746d744ce70acbfb';
const GIST_TOKEN = process.env.GIST_SECRET;

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

async function queryBalance(client, address) {
    const balance = await client.getBalance(address, 'ustars');
    return {
        address,
        balance: balance.amount,
        denom: balance.denom
    };
}

async function getGistFiles() {
    if (!EXECUTION_PLAN_GIST_ID || !GIST_TOKEN) {
        throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
    }

    console.log('Debug: Fetching execution plan from gist:', EXECUTION_PLAN_GIST_ID);
    const response = await fetch(`https://api.github.com/gists/${EXECUTION_PLAN_GIST_ID}`, {
        headers: {
            'Authorization': `token ${GIST_TOKEN}`,
            'Content-Type': 'application/json',
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to fetch gist: ${response.statusText}`);
    }

    const data = await response.json();
    console.log('Debug: Found gist files:', Object.keys(data.files));
    return data.files;
}

async function updateGist(balances) {
    if (!PROJECT_GIST_ID || !GIST_TOKEN) {
        console.log('Skipping gist update - missing PROJECT_GIST_ID or GIST_SECRET');
        return;
    }

    // Create badge data for each role
    const badgeData = {};
    balances.forEach(({ role, stars }) => {
        badgeData[`${role}-balance.json`] = {
            schemaVersion: 1,
            label: `${role} balance`,
            message: `${stars} STARS`,
            color: stars === '0.000000' ? 'red' : 'green',
            style: 'flat-square'
        };
    });

    // Also create a total balance badge
    const totalStars = balances.reduce((sum, { stars }) => sum + parseFloat(stars), 0);
    badgeData['total-balance.json'] = {
        schemaVersion: 1,
        label: 'total balance',
        message: `${totalStars.toFixed(6)} STARS`,
        color: totalStars > 0 ? 'blue' : 'red',
        style: 'flat-square'
    };

    console.log('Debug: Updating balance badges in gist:', PROJECT_GIST_ID);
    
    // Update gist
    const files = {};
    Object.entries(badgeData).forEach(([filename, content]) => {
        files[filename] = {
            content: JSON.stringify(content)
        };
    });

    const response = await fetch(`https://api.github.com/gists/${PROJECT_GIST_ID}`, {
        method: 'PATCH',
        headers: {
            'Authorization': `token ${GIST_TOKEN}`,
            'Content-Type': 'application/json',
        },
        body: JSON.stringify({ files })
    });

    if (!response.ok) {
        throw new Error(`Failed to update gist: ${response.statusText}`);
    }

    console.log('Balance badges updated successfully');
}

function getPreviousRun(gistFiles, filename) {
    const gistFile = gistFiles[filename];
    if (!gistFile) return null;
    
    try {
        return JSON.parse(gistFile.content);
    } catch (error) {
        console.warn(`Debug: Could not parse previous run from ${filename}:`, error);
        return null;
    }
}

async function getAddressesFromExecutionPlan() {
    const files = await getGistFiles();
    const addresses = {
        deployer: null,
        minter: null,
        owner: null,
        user: null
    };

    console.log('Debug: Looking for mosaic_tile_nft_deploy job output');
    
    // Look for the latest successful deploy job
    let latestRun = null;
    let latestTimestamp = 0;

    for (const [filename, file] of Object.entries(files)) {
        console.log('Debug: Checking file:', filename);
        if (filename.includes('mosaic_tile_nft_deploy')) {
            try {
                const run = getPreviousRun(files, filename);
                if (run && run.job?.data && run.timestamp) {
                    const timestamp = new Date(run.timestamp).getTime();
                    if (timestamp > latestTimestamp) {
                        latestTimestamp = timestamp;
                        latestRun = run;
                        console.log('Debug: Found newer deploy run from:', run.timestamp);
                    }
                }
            } catch (error) {
                console.warn(`Debug: Failed to parse file ${filename}:`, error);
            }
        }
    }

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

async function updateBalances() {
    const client = await CosmWasmClient.connect(RPC_ENDPOINT);
    
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
                stars: (parseInt(balance.balance) / 1_000_000).toFixed(6) // Convert ustars to STARS
            };
        })
    );

    // Update gist with balance data
    await updateGist(balances);
}

updateBalances().catch(console.error); 