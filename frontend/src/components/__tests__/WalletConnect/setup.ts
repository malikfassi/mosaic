import '@testing-library/jest-dom'
import { jest } from '@jest/globals'

// Initialize window.keplr mock
window.keplr = {
  enable: jest.fn(),
  getKey: jest.fn(),
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
}

// Helper function to simulate failed wallet connection
export const simulateFailedConnection = () => {
  window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
} 