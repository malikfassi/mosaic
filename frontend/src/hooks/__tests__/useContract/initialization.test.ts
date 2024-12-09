import { renderHook } from '@testing-library/react'
import { useContract } from '../../useContract'
import { STARGAZE_CHAIN_ID } from '@/config/chain'

describe('useContract Initialization', () => {
  it('initializes with default state', () => {
    const { result } = renderHook(() => useContract())
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBe(null)
  })

  it('uses default chain ID', () => {
    const { result } = renderHook(() => useContract())
    expect(result.current.chainId).toBe(STARGAZE_CHAIN_ID)
  })
}) 