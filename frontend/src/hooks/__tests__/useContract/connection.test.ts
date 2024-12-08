import { renderHook, act } from '@testing-library/react'
import { useContract } from '@/hooks/useContract'
import { mockKeplr } from './setup'

describe('useContract Connection', () => {
  beforeEach(() => {
    jest.clearAllMocks()
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
    const expectedError = new Error('Connection failed')
    jest.spyOn(mockKeplr, 'enable').mockRejectedValue(expectedError)
    
    const { result } = renderHook(() => useContract())
    
    await act(async () => {
      try {
        await result.current.connect()
      } catch (e) {
        // Error is expected to be thrown
      }
    })
    
    // Wait for next tick to ensure state is updated
    await act(async () => {
      await new Promise(resolve => setTimeout(resolve, 0))
    })
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.error).toEqual(expectedError)
  })
}) 