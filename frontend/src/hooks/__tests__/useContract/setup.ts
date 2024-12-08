import '@testing-library/jest-dom'
import { jest } from '@jest/globals'

beforeEach(() => {
  window.keplr.enable.mockResolvedValueOnce(undefined)
})

// Helper function to simulate successful connection
export const simulateSuccessfulConnection = () => {
  window.keplr.enable.mockResolvedValueOnce(undefined)
}

// Helper function to simulate connection error
export const simulateFailedConnection = () => {
  window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
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

// Add a dummy test to satisfy Jest
describe('useContract Setup', () => {
  it('exports helper functions', () => {
    expect(typeof simulateSuccessfulConnection).toBe('function')
    expect(typeof simulateFailedConnection).toBe('function')
  })
}) 