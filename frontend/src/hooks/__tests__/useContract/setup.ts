import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { AccountData } from '@cosmjs/proto-signing'
import { Keplr, Key, ChainInfo, KeplrSignOptions, OfflineAminoSigner, OfflineDirectSigner, StdSignDoc, SignDoc, AminoSignResponse, DirectSignResponse } from '@keplr-wallet/types'
import Long from 'long'

// Mock Keplr wallet
const mockKeplr = {
  enable: jest.fn<(chainIds: string | string[]) => Promise<void>>().mockResolvedValue(undefined),
  getKey: jest.fn<(chainId: string) => Promise<Key>>().mockResolvedValue({
    name: 'mock-key',
    algo: 'secp256k1',
    pubKey: new Uint8Array([1, 2, 3]),
    address: new Uint8Array([1, 2, 3]),
    bech32Address: 'stars1mock...',
    isNanoLedger: false,
    isKeystone: false,
    ethereumHexAddress: '0x1234567890abcdef'
  }),
  experimentalSuggestChain: jest.fn<(chainInfo: ChainInfo) => Promise<void>>().mockResolvedValue(undefined),
  getOfflineSigner: jest.fn<(chainId: string, signOptions?: KeplrSignOptions) => OfflineAminoSigner & OfflineDirectSigner>().mockReturnValue({
    getAccounts: jest.fn<() => Promise<readonly AccountData[]>>().mockResolvedValue([{
      address: 'stars1mock...',
      pubkey: new Uint8Array([1, 2, 3]),
      algo: 'secp256k1'
    }]),
    signDirect: jest.fn<(signerAddress: string, signDoc: SignDoc) => Promise<DirectSignResponse>>().mockResolvedValue({
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
    signAmino: jest.fn<(signerAddress: string, signDoc: StdSignDoc) => Promise<AminoSignResponse>>().mockResolvedValue({
      signed: {} as StdSignDoc,
      signature: {
        pub_key: {
          type: "tendermint/PubKeySecp256k1",
          value: "test"
        },
        signature: "test"
      }
    })
  })
} as jest.Mocked<Partial<Keplr>>;

// Set up global window.keplr mock
Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
  writable: true,
  configurable: true
});

// Export mock for test manipulation
export { mockKeplr };

// Helper function to simulate successful connection
export const simulateSuccessfulConnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  jest.spyOn(window.keplr, 'enable').mockResolvedValueOnce(undefined);
};

// Helper function to simulate successful reconnection
export const simulateSuccessfulReconnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  jest.spyOn(window.keplr, 'enable').mockResolvedValueOnce(undefined);
};

// Helper function to simulate connection error
export const simulateFailedConnection = () => {
  if (!window.keplr) {
    throw new Error('Keplr mock not initialized');
  }
  const error = new Error('Connection failed');
  jest.spyOn(window.keplr, 'enable').mockImplementationOnce(() => {
    return Promise.reject(error);
  });
  return error;
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