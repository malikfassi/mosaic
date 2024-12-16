import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';

const RPC_ENDPOINT = process.env.STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com:443';

export async function getClient() {
    return await CosmWasmClient.connect(RPC_ENDPOINT);
}

export async function queryBalance(client, address) {
    const balance = await client.getBalance(address, 'ustars');
    return {
        address,
        balance: balance.amount,
        denom: balance.denom
    };
}

export function convertUstarsToStars(ustars) {
    return (parseInt(ustars) / 1_000_000).toFixed(6);
} 