import { renderHook } from '@testing-library/react'
import { useContract } from '../../useContract'

describe('useContract Initialization', () => {
  it('initializes with default state', () => {
    const { result } = renderHook(() => useContract())
    
    expect(result.current.isConnected).toBe(false)
    expect(result.current.address).toBe('')
    expect(result.current.error).toBe(null)
  })

  it('initializes with custom chain ID', () => {
    const customChainId = 'stargaze-custom-1'
    const { result } = renderHook(() => useContract(customChainId))
    
    expect(result.current.chainId).toBe(customChainId)
  })
}) 