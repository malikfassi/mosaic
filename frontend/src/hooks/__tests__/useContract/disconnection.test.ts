import { renderHook, act } from '@testing-library/react'
import { useContract } from '../../useContract'

describe('useContract Disconnection', () => {
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
    window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
    await act(async () => {
      await result.current.connect()
    })
    expect(result.current.error).toBeTruthy()
    
    // Disconnect should clear error
    act(() => {
      result.current.disconnect()
    })
    
    expect(result.current.error).toBe(null)
  })
}) 