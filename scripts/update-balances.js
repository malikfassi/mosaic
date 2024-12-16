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

async function updateBalances() {
    const client = await CosmWasmClient.connect(RPC_ENDPOINT);
    
    // Get addresses from environment
    const addresses = {
        deployer: process.env.DEPLOYER_ADDRESS,
        minter: process.env.MINTER_ADDRESS,
        owner: process.env.OWNER_ADDRESS,
        user: process.env.USER_ADDRESS
    };

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