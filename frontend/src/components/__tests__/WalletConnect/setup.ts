import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { AccountData, DirectSignResponse, OfflineSigner, SignDoc } from '@cosmjs/proto-signing'
import { ChainInfo } from '@keplr-wallet/types'

// Mock offline signer
const mockOfflineSigner: OfflineSigner = {
  getAccounts: jest.fn().mockResolvedValue([{
    address: 'stars1mock...',
    pubkey: new Uint8Array([1, 2, 3]),
    algo: 'secp256k1'
  }] as AccountData[]),
  signDirect: jest.fn().mockResolvedValue({
    signed: {} as SignDoc,
    signature: {
      pub_key: {
        type: 'tendermint/PubKeySecp256k1',
        value: 'mock_pubkey'
      },
      signature: new Uint8Array([1, 2, 3])
    }
  } as DirectSignResponse)
}

// Initialize window.keplr mock
window.keplr = {
  enable: jest.fn().mockResolvedValue(undefined),
  getKey: jest.fn().mockResolvedValue({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  }),
  experimentalSuggestChain: jest.fn().mockResolvedValue(undefined),
  getOfflineSigner: jest.fn().mockReturnValue(mockOfflineSigner),
  getOfflineSignerOnlyAmino: jest.fn().mockReturnValue(mockOfflineSigner),
  getOfflineSignerAuto: jest.fn().mockResolvedValue(mockOfflineSigner),
  signArbitrary: jest.fn().mockResolvedValue({
    signature: new Uint8Array([1, 2, 3]),
    pub_key: {
      type: 'tendermint/PubKeySecp256k1',
      value: 'mock_pubkey'
    }
  })
}

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