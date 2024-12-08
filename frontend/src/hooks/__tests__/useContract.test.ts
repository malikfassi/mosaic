import { renderHook, act } from '@testing-library/react'
import { useContract } from '../useContract'

describe('useContract', () => {
  beforeEach(() => {
    // Reset mocks before each test
    window.keplr.enable.mockClear()
    window.keplr.getKey.mockClear()
  })

  it('initializes with default state', () => {
    const { result } = renderHook(() => useContract())
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBe(null)
  })

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

  it('disconnects wallet', async () => {
    const { result } = renderHook(() => useContract())
    
    await act(async () => {
      await result.current.connect()
    })
    
    expect(result.current.isConnected).toBe(true)
    
    act(() => {
      result.current.disconnect()
    })
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
  })
}) 