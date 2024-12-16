import { getGistFiles } from './gist.js';

export async function getDeployAddresses(gistId, token) {
    if (!gistId || !token) {
        throw new Error('Missing required parameters: gistId or token');
    }

    // Get gist files
    const gistFiles = await getGistFiles(gistId, token);
    console.log('Debug: Found gist files:', Object.keys(gistFiles));

    // Find all deploy job files
    const deployFiles = Object.entries(gistFiles)
        .filter(([filename]) => filename.includes('mosaic_tile_nft_deploy'))
        .map(([filename, file]) => {
            try {
                const content = JSON.parse(file.content);
                return {
                    filename,
                    timestamp: new Date(content.timestamp),
                    data: content.job.data
                };
            } catch (error) {
                console.warn(`Failed to parse file ${filename}:`, error);
                return null;
            }
        })
        .filter(file => file !== null);

    console.log('Debug: Found deploy files:', deployFiles.map(f => f.filename));

    if (deployFiles.length === 0) {
        throw new Error('No deploy job files found');
    }

    // Sort by timestamp and get the latest
    const latestDeploy = deployFiles.sort((a, b) => b.timestamp - a.timestamp)[0];
    console.log('Debug: Using latest deploy from:', latestDeploy.timestamp);
    
    // Extract addresses
    const addresses = {
        deployer: latestDeploy.data.deployer_address,
        minter: latestDeploy.data.minter_address,
        owner: latestDeploy.data.owner_address,
        user: latestDeploy.data.user_address,
        timestamp: latestDeploy.timestamp.toISOString()
    };

    // Validate addresses
    for (const [role, address] of Object.entries(addresses)) {
        if (!address && role !== 'timestamp') {
            console.log('Debug: Missing address for role:', role);
            console.log('Debug: Current addresses:', addresses);
            throw new Error(`Missing ${role} address in deploy data`);
        }
    }

    return addresses;
} 