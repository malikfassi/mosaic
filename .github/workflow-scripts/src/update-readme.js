import fs from 'fs/promises';
import { getDeployAddresses } from './utils/deploy-info.js';
import { generateComponentHashes } from './utils/component.js';
import { COMPONENTS, COMPONENT_TYPES } from './workflow-config.js';
import { getClient, queryAllBalances } from './utils/chain.js';

async function main() {
    try {
        // Get environment variables
        const {
            GITHUB_TOKEN,
            GIST_ID,
            DEPLOYER_ADDRESS,
            MINTER_ADDRESS,
            OWNER_ADDRESS,
            USER_ADDRESS
        } = process.env;

        if (!GITHUB_TOKEN || !GIST_ID) {
            throw new Error('Missing required environment variables');
        }

        // Calculate component hashes
        console.log('Calculating component hashes...');
        const hashes = generateComponentHashes(COMPONENTS);

        // Get latest deploy info and balances
        console.log('Getting latest deploy info and balances...');
        const [addresses, client] = await Promise.all([
            getDeployAddresses(GIST_ID, GITHUB_TOKEN),
            getClient()
        ]);

        const addressMap = {
            deployer: DEPLOYER_ADDRESS,
            minter: MINTER_ADDRESS,
            owner: OWNER_ADDRESS,
            user: USER_ADDRESS
        };

        const balances = await queryAllBalances(client, addressMap);

        // Prepare template data
        const templateData = {
            lastUpdated: new Date().toISOString(),
            hashes: {
                frontend: hashes[COMPONENT_TYPES.FRONTEND].slice(0, 8),
                mosaicTile: hashes[COMPONENT_TYPES.MOSAIC_TILE].slice(0, 8),
                mosaicVending: hashes[COMPONENT_TYPES.MOSAIC_VENDING].slice(0, 8)
            },
            deploy: {
                timestamp: addresses.timestamp,
                mosaicTileAddress: addresses.minter,
                mosaicVendingAddress: addresses.owner
            },
            balances: Object.entries(addressMap).reduce((acc, [role, address]) => ({
                ...acc,
                [role]: {
                    address,
                    balance: balances[role]
                }
            }), {})
        };

        // Read template
        console.log('Reading template...');
        const template = await fs.readFile('README.template.md', 'utf8');

        // Replace template variables
        let output = template
            .replace('{{ .LastUpdated }}', templateData.lastUpdated)
            .replace('{{ .Hashes.Frontend }}', templateData.hashes.frontend)
            .replace('{{ .Hashes.MosaicTile }}', templateData.hashes.mosaicTile)
            .replace('{{ .Hashes.MosaicVending }}', templateData.hashes.mosaicVending)
            .replace('{{ .Deploy.Timestamp }}', templateData.deploy.timestamp)
            .replace('{{ .Deploy.MosaicTileAddress }}', templateData.deploy.mosaicTileAddress)
            .replace('{{ .Deploy.MosaicVendingAddress }}', templateData.deploy.mosaicVendingAddress);

        // Replace balance section
        const balanceRows = Object.entries(templateData.balances)
            .map(([role, data]) => `| ${role} | \`${data.address}\` | ${data.balance} |`)
            .join('\n');

        output = output.replace(/\{\{- range \$role, \$data := \.Balances \}\}[\s\S]*?\{\{- end \}\}/, balanceRows);

        // Write output
        console.log('Writing README...');
        await fs.writeFile('README.md', output);
        console.log('README.md updated successfully');

    } catch (error) {
        console.error('Error:', error);
        process.exit(1);
    }
}

main(); 