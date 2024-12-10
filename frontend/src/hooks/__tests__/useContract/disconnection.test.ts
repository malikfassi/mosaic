import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { simulateFailedConnection, simulateSuccessfulConnection } from './setup'

// Helper function to wait for state updates
const waitForStateUpdate = async () => {
  await act(async () => {
    await new Promise(resolve => setTimeout(resolve, 100))
  })
}

describe('useContract Disconnection', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('disconnects wallet', async () => {
    const { result } = renderHook(() => useContract())
    
    // First connect
    simulateSuccessfulConnection()
    await act(async () => {
      await result.current.connect()
    })
    await waitForStateUpdate()
    expect(result.current.isConnected).toBe(true)
    
    // Then disconnect
    act(() => {
      result.current.disconnect()
    })
    
    // Wait for state updates to complete
    await waitForStateUpdate()
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBe(null)
  })

  it('clears error state on disconnect', async () => {
    const { result } = renderHook(() => useContract())
    
    // Simulate failed connection
    const expectedError = simulateFailedConnection()
    
    await act(async () => {
      try {
        await result.current.connect()
      } catch (error) {
        // Error is expected, we'll verify the state below
        expect(error).toEqual(expectedError)
      }
    })
    
    // Wait for state updates to complete
    await waitForStateUpdate()
    
    // Verify error state is set
    expect(result.current.error).toEqual(expectedError)
    
    // Then disconnect
    act(() => {
      result.current.disconnect()
    })
    
    // Wait for state updates to complete
    await waitForStateUpdate()
    
    // Verify error state is cleared
    expect(result.current.error).toBe(null)
  })
}) 