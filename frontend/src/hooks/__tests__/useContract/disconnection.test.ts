import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { mockKeplr } from './setup'

describe('useContract Disconnection', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('disconnects wallet', async () => {
    const { result } = renderHook(() => useContract())
    
    // First connect
    await act(async () => {
      await result.current.connect()
    })
    expect(result.current.isConnected).toBe(true)
    
    // Then disconnect
    act(() => {
      result.current.disconnect()
    })
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBe(null)
  })

  it('clears error state on disconnect', async () => {
    const { result } = renderHook(() => useContract())
    
    // Simulate failed connection
    const expectedError = new Error('Connection failed')
    jest.spyOn(mockKeplr, 'enable').mockRejectedValueOnce(expectedError)
    await act(async () => {
      try {
        await result.current.connect()
      } catch {
        // Expected error
      }
    })
    expect(result.current.error).toEqual(expectedError)
    
    // Then disconnect
    act(() => {
      result.current.disconnect()
    })
    
    expect(result.current.error).toBe(null)
  })
}) 