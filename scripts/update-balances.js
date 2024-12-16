import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { fileURLToPath } from 'url';
import path from 'path';
import fetch from 'node-fetch';

const RPC_ENDPOINT = process.env.STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com:443';
const GIST_ID = process.env.GIST_ID || 'c67eb85b7002c9e7746d744ce70acbfb';
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
    if (!GIST_ID || !GIST_TOKEN) {
        throw new Error('Missing required environment variables: GIST_ID or GIST_SECRET');
    }

    const response = await fetch(`https://api.github.com/gists/${GIST_ID}`, {
        headers: {
            'Authorization': `token ${GIST_TOKEN}`,
            'Content-Type': 'application/json',
        }
    });

    if (!response.ok) {
        throw new Error(`Failed to fetch gist: ${response.statusText}`);
    }

    const data = await response.json();
    return data.files;
}

async function updateGist(balances) {
    if (!GIST_ID || !GIST_TOKEN) {
        console.log('Skipping gist update - missing GIST_ID or GIST_TOKEN');
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

    // Update gist
    const files = {};
    Object.entries(badgeData).forEach(([filename, content]) => {
        files[filename] = {
            content: JSON.stringify(content)
        };
    });

    const response = await fetch(`https://api.github.com/gists/${GIST_ID}`, {
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

    console.log('Gist updated successfully');
}

async function getAddressesFromExecutionPlan() {
    const files = await getGistFiles();
    const addresses = {
        deployer: null,
        minter: null,
        owner: null,
        user: null
    };

    // Look for the mosaic_tile_nft_deploy job output
    for (const [filename, file] of Object.entries(files)) {
        if (filename.includes('mosaic_tile_nft_deploy')) {
            try {
                const content = JSON.parse(file.content);
                if (content.job?.data) {
                    const data = content.job.data;
                    addresses.deployer = data.deployer_address;
                    addresses.minter = data.minter_address;
                    addresses.owner = data.owner_address;
                    addresses.user = data.user_address;
                    break;
                }
            } catch (error) {
                console.warn(`Failed to parse file ${filename}:`, error);
            }
        }
    }

    // Validate that we have all addresses
    for (const [role, address] of Object.entries(addresses)) {
        if (!address) {
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