export const STARGAZE_CHAIN_ID = 'stargaze-1';

export const getStargazeChainInfo = () => ({
  chainId: STARGAZE_CHAIN_ID,
  chainName: 'Stargaze',
  rpc: 'https://stargaze-rpc.polkachu.com',
  rest: 'https://stargaze-api.polkachu.com',
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
  features: ['stargate', 'ibc-transfer', 'no-legacy-stdTx', 'ibc-go'],
}); 