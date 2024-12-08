import { renderHook, act } from '@testing-library/react'
import { useContract } from '../../useContract'

describe('useContract Connection', () => {
  it('connects to wallet successfully', async () => {
    const { result } = renderHook(() => useContract())
    
    await act(async () => {
      await result.current.connect()
    })
    
    expect(result.current.isConnected).toBe(true)
    expect(result.current.address).toBe('stars1mock...')
    expect(result.current.error).toBe(null)
  })

  it('handles connection errors', async () => {
    window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
    
    const { result } = renderHook(() => useContract())
    
    await act(async () => {
      await result.current.connect()
    })
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toBeTruthy()
  })
}) 