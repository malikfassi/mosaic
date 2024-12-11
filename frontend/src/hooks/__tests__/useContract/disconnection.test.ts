import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { setupHookTest, simulateSuccessfulConnection, simulateFailedConnection } from './setup'
import { Keplr } from '@keplr-wallet/types'

// Helper function to wait for state updates
const waitForStateUpdate = async () => {
  await act(async () => {
    await new Promise(resolve => setTimeout(resolve, 100))
  })
}

describe('useContract Disconnection', () => {
  const { wrapper } = setupHookTest()

  beforeEach(() => {
    jest.clearAllMocks()
    // Ensure Keplr is available for each test with all required functions
    if (!window.keplr) {
      window.keplr = {
        enable: jest.fn().mockResolvedValue(undefined),
        getKey: jest.fn(),
        getOfflineSigner: jest.fn(),
        experimentalSuggestChain: jest.fn().mockResolvedValue(undefined),
      } as Partial<Keplr> as Keplr;
    }
  })

  it('disconnects wallet', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })
    
    // First connect
    simulateSuccessfulConnection()
    await act(async () => {
      await result.current.connect()
    })
    await waitForStateUpdate()
    expect(result.current.isConnected).toBe(true)
    
    // Then disconnect
    await act(async () => {
      result.current.disconnect()
      // Wait for state update within the act block
      await waitForStateUpdate()
    })
    
    // Verify disconnected state
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBeNull()
    expect(result.current.client).toBeNull()
  })

  it('clears error state on disconnect', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })
    
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
    await act(async () => {
      result.current.disconnect()
      // Wait for state update within the act block
      await waitForStateUpdate()
    })
    
    // Verify error state is cleared
    expect(result.current.error).toBeNull()
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.client).toBeNull()
  })
}) 