import fs from 'fs/promises';
import { getDeployAddresses, saveDeployInfo, loadDeployInfo } from './utils/deploy-info.js';
import { generateComponentHashes } from './utils/component.js';
import { COMPONENTS, COMPONENT_TYPES } from './workflow-config.js';
import { getClient, queryAllBalances } from './utils/chain.js';

async function generateReadmeData() {
    // Get environment variables
    const {
        GIST_SECRET,
        GIST_ID,
        DEPLOYER_ADDRESS,
        MINTER_ADDRESS,
        OWNER_ADDRESS,
        USER_ADDRESS
    } = process.env;

    if (!GIST_SECRET || !GIST_ID) {
        throw new Error('Missing required environment variables');
    }

    // Calculate component hashes
    console.log('Calculating component hashes...');
    const hashes = generateComponentHashes(COMPONENTS);

    // Get latest deploy info and balances
    console.log('Getting latest deploy info and balances...');
    const [addresses, client] = await Promise.all([
        getDeployAddresses(GIST_ID, GIST_SECRET),
        getClient()
    ]);

    const addressMap = {
        deployer: DEPLOYER_ADDRESS,
        minter: MINTER_ADDRESS,
        owner: OWNER_ADDRESS,
        user: USER_ADDRESS
    };

    const balances = await queryAllBalances(client, addressMap);

    // Prepare data
    const data = {
        lastUpdated: new Date().toISOString(),
        hashes: {
            frontend: hashes[COMPONENT_TYPES.FRONTEND].slice(0, 8),
            mosaicTile: hashes[COMPONENT_TYPES.MOSAIC_TILE].slice(0, 8),
        },
        deploy: {
            timestamp: addresses.timestamp,
            mosaicTileAddress: addresses.minter,
        },
        balances: Object.entries(addressMap).reduce((acc, [role, address]) => ({
            ...acc,
            [role]: {
                address,
                balance: balances[role]
            }
        }), {})
    };

    return data;
}

async function updateReadme(data) {
    // Read template
    console.log('Reading template...');
    const template = await fs.readFile('README.template.md', 'utf8');

    // Replace template variables
    let output = template
        .replace('{{ .LastUpdated }}', data.lastUpdated)
        .replace('{{ .Hashes.Frontend }}', data.hashes.frontend)
        .replace('{{ .Hashes.MosaicTile }}', data.hashes.mosaicTile)
        .replace('{{ .Deploy.Timestamp }}', data.deploy.timestamp)
        .replace('{{ .Deploy.MosaicTileAddress }}', data.deploy.mosaicTileAddress)

    // Replace balance section
    const balanceRows = Object.entries(data.balances)
        .map(([role, data]) => `| ${role} | \`${data.address}\` | ${data.balance} |`)
        .join('\n');

    output = output.replace(/\{\{- range \$role, \$data := \.Balances \}\}[\s\S]*?\{\{- end \}\}/, balanceRows);

    // Write output
    console.log('Writing README...');
    await fs.writeFile('README.md', output);
    console.log('README.md updated successfully');
}

async function main() {
    try {
        // Generate data
        const data = await generateReadmeData();

        // Save data to JSON file
        await saveDeployInfo(data);

        // Update README with the data
        await updateReadme(data);

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

main(); 