import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { DirectSignResponse } from '@cosmjs/proto-signing'
import { Keplr, Key, StdSignature, AminoSignResponse, OfflineAminoSigner, OfflineDirectSigner, StdSignDoc, SignDoc } from '@keplr-wallet/types'
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

// Initialize window.keplr mock with proper Jest mock types
const mockKeplr = {
  version: "0.12.22",
  mode: "extension",
  defaultOptions: {
    sign: {
      preferNoSetFee: false,
      preferNoSetMemo: false,
    },
  },
  enable: jest.fn(),
  getKey: jest.fn(),
  experimentalSuggestChain: jest.fn(),
  getOfflineSigner: jest.fn(),
  getOfflineSignerOnlyAmino: jest.fn(),
  getOfflineSignerAuto: jest.fn(),
  signArbitrary: jest.fn(),
} as unknown as jest.Mocked<Keplr>;

// Set up mock implementations
mockKeplr.enable.mockImplementation(async () => undefined);
mockKeplr.getKey.mockImplementation(async (): Promise<Key> => ({
  name: 'mock-key',
  algo: 'secp256k1',
  pubKey: new Uint8Array([1, 2, 3]),
  address: new Uint8Array([1, 2, 3]),
  bech32Address: 'stars1mock...',
  isNanoLedger: false,
  isKeystone: false,
  ethereumHexAddress: '0x1234567890abcdef'
}));
mockKeplr.experimentalSuggestChain.mockImplementation(async () => undefined);
mockKeplr.getOfflineSigner.mockImplementation(() => mockOfflineSigner);
mockKeplr.getOfflineSignerOnlyAmino.mockImplementation(() => ({
  getAccounts: mockOfflineSigner.getAccounts,
  signAmino: mockOfflineSigner.signAmino
}));
mockKeplr.getOfflineSignerAuto.mockImplementation(async () => mockOfflineSigner);
mockKeplr.signArbitrary.mockImplementation(async (): Promise<StdSignature> => ({
  pub_key: {
    type: 'tendermint/PubKeySecp256k1',
    value: 'mock_pubkey'
  },
  signature: 'mock_signature'
}));

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
    throw new Error('Keplr mock not initialized');
  }
  window.keplr.enable.mockImplementation(async () => undefined);
};

export const simulateSuccessfulReconnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  window.keplr.enable.mockImplementation(async () => undefined);
};

export const simulateFailedConnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  window.keplr.enable.mockImplementation(async () => {
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