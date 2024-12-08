import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { OfflineSigner } from '@cosmjs/proto-signing'

// Mock offline signer
const mockOfflineSigner: OfflineSigner = {
  getAccounts: jest.fn().mockResolvedValue([{
    address: 'stars1mock...',
    pubkey: new Uint8Array([1, 2, 3])
  }]),
  signDirect: jest.fn(),
}

// Initialize window.keplr mock
window.keplr = {
  enable: jest.fn(),
  getKey: jest.fn(),
  experimentalSuggestChain: jest.fn(),
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