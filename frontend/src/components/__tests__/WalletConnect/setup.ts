import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { AccountData, OfflineSigner } from '@cosmjs/proto-signing'
import { ChainInfo } from '@keplr-wallet/types'

// Mock offline signer
const mockOfflineSigner: OfflineSigner = {
  getAccounts: jest.fn<() => Promise<readonly AccountData[]>>().mockResolvedValue([{
    address: 'stars1mock...',
    pubkey: new Uint8Array([1, 2, 3]),
    algo: 'secp256k1'
  }]),
  signDirect: jest.fn<(signerAddress: string, signDoc: any) => Promise<any>>().mockResolvedValue({
    signed: {},
    signature: {
      pub_key: {
        type: 'tendermint/PubKeySecp256k1',
        value: 'mock_pubkey'
      },
      signature: 'mock_signature'
    }
  })
}

// Initialize window.keplr mock
Object.defineProperty(window, 'keplr', {
  value: {
    enable: jest.fn<(chainIds: string | string[]) => Promise<void>>().mockResolvedValue(undefined),
    getKey: jest.fn<(chainId: string) => Promise<{ bech32Address: string; pubKey: Uint8Array }>>().mockResolvedValue({
      bech32Address: 'stars1mock...',
      pubKey: new Uint8Array([1, 2, 3]),
    }),
    experimentalSuggestChain: jest.fn<(chainInfo: ChainInfo) => Promise<void>>().mockResolvedValue(undefined),
    getOfflineSigner: jest.fn<(chainId: string) => OfflineSigner>().mockReturnValue(mockOfflineSigner),
    getOfflineSignerOnlyAmino: jest.fn<(chainId: string) => OfflineSigner>().mockReturnValue(mockOfflineSigner),
    getOfflineSignerAuto: jest.fn<(chainId: string) => Promise<OfflineSigner>>().mockResolvedValue(mockOfflineSigner),
    signArbitrary: jest.fn<(chainId: string, signer: string, data: string) => Promise<{
      signature: Uint8Array;
      pub_key: { type: string; value: string };
    }>>().mockResolvedValue({
      signature: new Uint8Array([1, 2, 3]),
      pub_key: {
        type: 'tendermint/PubKeySecp256k1',
        value: 'mock_pubkey'
      }
    })
  },
  writable: true,
  configurable: true
})

beforeEach(() => {
  // Reset all mocks before each test
  jest.clearAllMocks()
})

// Helper function to simulate successful wallet connection
export const simulateSuccessfulConnection = async () => {
  window.keplr.enable.mockResolvedValueOnce(undefined)
  window.keplr.getKey.mockResolvedValueOnce({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  })
  window.keplr.experimentalSuggestChain.mockResolvedValueOnce(undefined)
}

// Helper function to simulate failed wallet connection
export const simulateFailedConnection = () => {
  window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
} 