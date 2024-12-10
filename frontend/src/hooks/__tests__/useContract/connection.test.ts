import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { setupHookTest } from './setup'
import { STARGAZE_CHAIN_ID } from '@/config/chain'

describe('useContract Hook - Connection Tests', () => {
  const { wrapper } = setupHookTest()

  beforeEach(() => {
    // Reset Keplr mock before each test
    if (window.keplr) {
      (window.keplr.enable as jest.Mock).mockClear();
      (window.keplr.getKey as jest.Mock).mockClear();
    }
  })

  it('should connect to wallet successfully', async () => {
    const { result } = renderHook(() => useContract(), { wrapper })

    await act(async () => {
      await result.current.connect()
    })

    expect(window.keplr?.enable).toHaveBeenCalledWith(STARGAZE_CHAIN_ID)
    expect(result.current.isConnected).toBe(true)
    expect(result.current.error).toBeNull()
  })

  it('should handle connection errors gracefully', async () => {
    if (window.keplr) {
      (window.keplr.enable as jest.Mock).mockRejectedValueOnce(new Error('Connection failed'))
    }
    
    const { result } = renderHook(() => useContract(), { wrapper })

    await act(async () => {
      await result.current.connect()
    })

    expect(window.keplr?.enable).toHaveBeenCalled()
    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toBeTruthy()
  })

  it('should handle missing Keplr wallet', async () => {
    // Temporarily remove Keplr
    const originalKeplr = window.keplr
    const tempWindow = window as any
    delete tempWindow.keplr

    const { result } = renderHook(() => useContract(), { wrapper })

    await act(async () => {
      await result.current.connect()
    })

    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toBeTruthy()

    // Restore Keplr
    window.keplr = originalKeplr
  })
}) 