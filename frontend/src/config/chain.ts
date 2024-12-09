export const STARGAZE_CHAIN_ID = process.env.NEXT_PUBLIC_STARGAZE_CHAIN_ID || 'elgafar-1';

export const getStargazeChainInfo = () => ({
  chainId: STARGAZE_CHAIN_ID,
  chainName: STARGAZE_CHAIN_ID === 'elgafar-1' ? 'Stargaze Testnet' : 'Stargaze',
  rpc: process.env.NEXT_PUBLIC_STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com',
  rest: process.env.NEXT_PUBLIC_STARGAZE_REST || 'https://rest.elgafar-1.stargaze-apis.com',
  bip44: {
    coinType: 118,
  },
  bech32Config: {
    bech32PrefixAccAddr: 'stars',
    bech32PrefixAccPub: 'starspub',
    bech32PrefixValAddr: 'starsvaloper',
    bech32PrefixValPub: 'starsvaloperpub',
    bech32PrefixConsAddr: 'starsvalcons',
    bech32PrefixConsPub: 'starsvalconspub',
  },
  currencies: [
    {
      coinDenom: 'STARS',
      coinMinimalDenom: 'ustars',
      coinDecimals: 6,
      coinGeckoId: 'stargaze',
    },
  ],
  feeCurrencies: [
    {
      coinDenom: 'STARS',
      coinMinimalDenom: 'ustars',
      coinDecimals: 6,
      coinGeckoId: 'stargaze',
      gasPriceStep: {
        low: 0.01,
        average: 0.025,
        high: 0.04,
      },
    },
  ],
  stakeCurrency: {
    coinDenom: 'STARS',
    coinMinimalDenom: 'ustars',
    coinDecimals: 6,
    coinGeckoId: 'stargaze',
  },
  features: ['ibc-transfer', 'ibc-go'],
}); 