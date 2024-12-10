import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { setupHookTest, simulateSuccessfulConnection, simulateFailedConnection } from './setup'
import { STARGAZE_CHAIN_ID } from '@/config/chain'

// Helper function to wait for state updates
const waitForStateUpdate = async () => {
  await act(async () => {
    await new Promise(resolve => setTimeout(resolve, 100))
  })
}

describe('useContract Hook - Connection Tests', () => {
  const { wrapper } = setupHookTest()

  beforeEach(() => {
    // Clear all mock implementations before each test
    if (window.keplr) {
      Object.values(window.keplr).forEach(mock => {
        if (typeof mock === 'function' && 'mockClear' in mock) {
          mock.mockClear();
        }
      });
    }
  })

  it('should connect to wallet successfully', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })

    // Setup successful connection before connecting
    simulateSuccessfulConnection()
    await act(async () => {
      await result.current.connect()
    })

    expect(window.keplr?.enable).toHaveBeenCalledWith(STARGAZE_CHAIN_ID)
    expect(window.keplr?.experimentalSuggestChain).toHaveBeenCalled()
    expect(result.current.isConnected).toBe(true)
    expect(result.current.error).toBeNull()
  })

  it('should handle connection errors gracefully', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })

    // Setup failed connection
    const expectedError = simulateFailedConnection()

    await act(async () => {
      try {
        await result.current.connect()
      } catch (error) {
        expect(error).toEqual(expectedError)
      }
    })

    // Wait for state updates to complete
    await waitForStateUpdate()

    expect(window.keplr?.enable).toHaveBeenCalled()
    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toBeTruthy()
  })

  it('should handle missing Keplr wallet', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })

    // Temporarily remove Keplr
    const originalKeplr = window.keplr
    // Cast to unknown first to avoid type checking during the assignment
    const tempWindow = window as unknown as { keplr?: typeof window.keplr }
    tempWindow.keplr = undefined

    await act(async () => {
      try {
        await result.current.connect()
      } catch (error) {
        expect(error).toEqual(new Error('Please install Keplr extension'))
      }
    })

    // Wait for state updates to complete
    await waitForStateUpdate()

    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toBeTruthy()

    // Restore Keplr
    window.keplr = originalKeplr
  })
}) 