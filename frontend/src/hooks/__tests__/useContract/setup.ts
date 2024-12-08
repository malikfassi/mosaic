import '@testing-library/jest-dom'

beforeEach(() => {
  // Reset all mocks before each test
  window.keplr.enable.mockClear()
  window.keplr.getKey.mockClear()
})

// Helper function to simulate successful connection
export const simulateSuccessfulConnection = () => {
  window.keplr.enable.mockResolvedValueOnce(true)
  window.keplr.getKey.mockResolvedValueOnce({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  })
}

// Helper function to simulate connection error
export const simulateConnectionError = (errorMessage: string = 'Connection failed') => {
  window.keplr.enable.mockRejectedValueOnce(new Error(errorMessage))
}

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