import '@testing-library/jest-dom'

beforeEach(() => {
  // Reset all mocks before each test
  window.keplr.enable.mockClear()
  window.keplr.getKey.mockClear()
})

// Helper function to simulate successful wallet connection
export const simulateSuccessfulConnection = async () => {
  window.keplr.enable.mockResolvedValueOnce(true)
  window.keplr.getKey.mockResolvedValueOnce({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  })
}

// Helper function to simulate failed wallet connection
export const simulateFailedConnection = () => {
  window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
} 