const { CosmWasmClient } = require('@cosmjs/cosmwasm-stargate');
const fs = require('fs').promises;
const path = require('path');

const RPC_ENDPOINT = process.env.STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com:443';

async function queryBalance(client, address) {
    const balance = await client.getBalance(address, 'ustars');
    return {
        address,
        balance: balance.amount,
        denom: balance.denom
    };
}

async function updateReadme() {
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

    // Create markdown table
    const timestamp = new Date().toISOString();
    const table = [
        '## Account Balances',
        '',
        `*Last updated: ${timestamp}*`,
        '',
        '| Role | Address | Balance (STARS) |',
        '|------|---------|----------------|',
        ...balances.map(({ role, address, stars }) => 
            `| ${role} | \`${address}\` | ${stars} |`
        ),
        ''
    ].join('\n');

    // Read current README
    const readmePath = path.join(process.env.GITHUB_WORKSPACE, 'README.md');
    let readme = await fs.readFile(readmePath, 'utf8');

    // Replace or append balance section
    const balanceSection = /## Account Balances[\s\S]*?(?=##|$)/;
    if (balanceSection.test(readme)) {
        readme = readme.replace(balanceSection, table);
    } else {
        readme = `${readme}\n${table}`;
    }

    // Write updated README
    await fs.writeFile(readmePath, readme);
}

updateReadme().catch(console.error); 