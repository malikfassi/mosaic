import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { Keplr, AminoSignResponse, OfflineAminoSigner, OfflineDirectSigner, StdSignDoc, SignDoc, DirectSignResponse, ChainInfo } from '@keplr-wallet/types'
import Long from 'long'

// Mock offline signer with both Direct and Amino signing capabilities
const mockOfflineSigner: OfflineAminoSigner & OfflineDirectSigner = {
  getAccounts: async () => [{
    address: 'stars1mock...',
    pubkey: new Uint8Array([1, 2, 3]),
    algo: 'secp256k1'
  }],
  signDirect: async (_signerAddress: string, _signDoc: SignDoc): Promise<DirectSignResponse> => ({
    signed: {
      bodyBytes: new Uint8Array(),
      authInfoBytes: new Uint8Array(),
      chainId: "test-chain",
      accountNumber: Long.fromNumber(0)
    },
    signature: {
      pub_key: {
        type: "tendermint/PubKeySecp256k1",
        value: "test"
      },
      signature: "test"
    }
  }),
  signAmino: async (_signerAddress: string, _signDoc: StdSignDoc): Promise<AminoSignResponse> => ({
    signed: _signDoc,
    signature: {
      pub_key: {
        type: "tendermint/PubKeySecp256k1",
        value: "test"
      },
      signature: "test"
    }
  })
};

// Create a mock implementation of all Keplr methods
const mockKeplr = {
  version: "0.12.22",
  mode: "extension",
  defaultOptions: {
    sign: {
      preferNoSetFee: false,
      preferNoSetMemo: false,
    },
  },
  enable: jest.fn().mockImplementation(async () => undefined),
  disable: jest.fn().mockImplementation(async () => undefined),
  getKey: jest.fn().mockImplementation(async () => ({
    name: 'mock-key',
    algo: 'secp256k1',
    pubKey: new Uint8Array([1, 2, 3]),
    address: new Uint8Array([1, 2, 3]),
    bech32Address: 'stars1mock...',
    isNanoLedger: false,
    isKeystone: false,
    ethereumHexAddress: '0x1234567890abcdef'
  })),
  getKeysSettled: jest.fn().mockImplementation(async () => []),
  signAmino: jest.fn().mockImplementation(async () => ({
    signed: {} as StdSignDoc,
    signature: {
      pub_key: { type: 'tendermint/PubKeySecp256k1', value: 'mock' },
      signature: 'mock'
    }
  })),
  signDirect: jest.fn().mockImplementation(async () => ({
    signed: {
      bodyBytes: new Uint8Array(),
      authInfoBytes: new Uint8Array(),
      chainId: 'mock-chain',
      accountNumber: Long.fromNumber(0)
    },
    signature: {
      pub_key: { type: 'tendermint/PubKeySecp256k1', value: 'mock' },
      signature: 'mock'
    }
  })),
  sendTx: jest.fn().mockImplementation(async () => new Uint8Array()),
  experimentalSuggestChain: jest.fn().mockImplementation(async () => undefined),
  getOfflineSigner: jest.fn().mockImplementation(() => mockOfflineSigner),
  getOfflineSignerOnlyAmino: jest.fn().mockImplementation(() => ({
    getAccounts: mockOfflineSigner.getAccounts,
    signAmino: mockOfflineSigner.signAmino
  })),
  getOfflineSignerAuto: jest.fn().mockImplementation(async () => mockOfflineSigner),
  signArbitrary: jest.fn().mockImplementation(async () => ({
    pub_key: {
      type: 'tendermint/PubKeySecp256k1',
      value: 'mock_pubkey'
    },
    signature: 'mock_signature'
  })),
  verifyArbitrary: jest.fn().mockImplementation(async () => true),
  signEthereum: jest.fn().mockImplementation(async () => new Uint8Array()),
  getEnigmaPubKey: jest.fn().mockImplementation(async () => new Uint8Array()),
  getEnigmaTxEncryptionKey: jest.fn().mockImplementation(async () => new Uint8Array()),
  enigmaEncrypt: jest.fn().mockImplementation(async () => new Uint8Array()),
  enigmaDecrypt: jest.fn().mockImplementation(async () => new Uint8Array()),
  getSecret20ViewingKey: jest.fn().mockImplementation(async () => 'mock_key'),
  __core__: jest.fn() as jest.Mock,
  changeKeyRingName: jest.fn().mockImplementation(async () => 'mock_name'),
  sign: jest.fn().mockImplementation(async () => new Uint8Array()),
  signEthereumTypedData: jest.fn().mockImplementation(async () => 'mock_signature'),
  suggestToken: jest.fn().mockImplementation(async () => undefined),
  ping: jest.fn().mockImplementation(async () => undefined),
  addChain: jest.fn().mockImplementation(async () => undefined),
  deleteChain: jest.fn().mockImplementation(async () => undefined),
  updateChain: jest.fn().mockImplementation(async () => undefined),
  hasChain: jest.fn().mockImplementation(async () => true),
  getChain: jest.fn().mockImplementation(async () => ({} as ChainInfo)),
  getChains: jest.fn().mockImplementation(async () => []),
  getChainCrypto: jest.fn().mockImplementation(async () => ({})),
  getChainFeatures: jest.fn().mockImplementation(async () => []),
  getChainName: jest.fn().mockImplementation(async () => 'mock-chain'),
  getChainId: jest.fn().mockImplementation(async () => 'mock-chain-id'),
  getChainRpc: jest.fn().mockImplementation(async () => 'http://mock-rpc'),
  getChainRest: jest.fn().mockImplementation(async () => 'http://mock-rest'),
  getChainSymbol: jest.fn().mockImplementation(async () => 'MOCK'),
  getChainCurrency: jest.fn().mockImplementation(async () => ({ coinDenom: 'MOCK', coinMinimalDenom: 'umock', coinDecimals: 6 })),
  getChainExplorer: jest.fn().mockImplementation(async () => 'http://mock-explorer'),
  getChainStakeCurrency: jest.fn().mockImplementation(async () => ({ coinDenom: 'MOCK', coinMinimalDenom: 'umock', coinDecimals: 6 })),
  getChainFeeCurrencies: jest.fn().mockImplementation(async () => [{ coinDenom: 'MOCK', coinMinimalDenom: 'umock', coinDecimals: 6 }]),
  getChainBech32Config: jest.fn().mockImplementation(async () => ({ bech32PrefixAccAddr: 'mock' })),
  getChainBip44: jest.fn().mockImplementation(async () => ({ coinType: 118 })),
  getChainGasPriceStep: jest.fn().mockImplementation(async () => ({ low: 0.01, average: 0.025, high: 0.04 })),
  signDirectAux: jest.fn().mockImplementation(async () => ({} as DirectSignResponse)),
  signICNSAdr36: jest.fn().mockImplementation(async () => ({} as AminoSignResponse)),
  getEnigmaUtils: jest.fn().mockImplementation(() => ({})),
  enigmaIsNewApi: jest.fn().mockImplementation(() => true),
  starknet: {} as Record<string, unknown>
} as unknown as jest.Mocked<Keplr>;

Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
  writable: true,
  configurable: true
});

beforeEach(() => {
  jest.clearAllMocks();
});

// Helper functions
export const simulateSuccessfulConnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr not initialized');
  }
  return window.keplr;
};

export const simulateSuccessfulReconnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  mockKeplr.enable.mockImplementation(async () => undefined);
};

export const simulateFailedConnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  mockKeplr.enable.mockImplementation(async () => {
    throw new Error('Connection failed');
  });
};

// Helper function to simulate successful transaction
export const simulateSuccessfulTransaction = () => {
  return {
    transactionHash: '0x123...',
    code: 0,
    rawLog: 'success',
  }
}

// Helper function to simulate failed transaction
export const simulateFailedTransaction = (errorMessage: string = 'Transaction failed') => {
  throw new Error(errorMessage)
}

// Add a dummy test to satisfy Jest
describe('useContract Setup', () => {
  it('exports helper functions', () => {
    expect(typeof simulateSuccessfulConnection).toBe('function')
    expect(typeof simulateFailedConnection).toBe('function')
  })
}) 