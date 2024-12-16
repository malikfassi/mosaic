import { COMPONENTS, COMPONENT_TYPES } from '../workflow-config.js';
import { getGistFiles, getLatestRun } from './gist.js';
import { calculateComponentHash } from './component.js';

export async function getDeployAddresses(gistId, token) {
    if (!gistId || !token) {
        throw new Error('Missing required parameters: gistId or token');
    }

    // Get gist files and calculate component hash
    const gistFiles = await getGistFiles(gistId, token);
    const componentHash = calculateComponentHash(COMPONENTS[COMPONENT_TYPES.MOSAIC_TILE]);
    console.log('Debug: Component hash for mosaic tile:', componentHash);

    // Find latest successful deploy run
    const deployJobPattern = `mosaic_tile_nft_deploy.${componentHash}`;
    console.log('Debug: Looking for deploy job with pattern:', deployJobPattern);
    
    const latestRun = getLatestRun(gistFiles, deployJobPattern);
    if (!latestRun) {
        throw new Error('No successful deploy job found');
    }

    console.log('Debug: Found deploy data from run:', latestRun.timestamp);
    const data = latestRun.job.data;
    
    // Extract addresses
    const addresses = {
        deployer: data.deployer_address,
        minter: data.minter_address,
        owner: data.owner_address,
        user: data.user_address
    };

    // Validate addresses
    for (const [role, address] of Object.entries(addresses)) {
        if (!address) {
            console.log('Debug: Missing address for role:', role);
            console.log('Debug: Current addresses:', addresses);
            throw new Error(`Missing ${role} address in deploy data`);
        }
    }

    return addresses;
} 