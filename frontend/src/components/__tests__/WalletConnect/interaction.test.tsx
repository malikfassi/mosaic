import React from 'react'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import '@testing-library/jest-dom'
import WalletConnect from '../../WalletConnect'
import { simulateSuccessfulConnection, simulateFailedConnection } from './setup'
import { toast } from 'react-hot-toast'

jest.mock('react-hot-toast', () => ({
  __esModule: true,
  toast: {
    success: jest.fn(),
    error: jest.fn(),
  },
}))

describe('WalletConnect Interactions', () => {
  const originalError = console.error
  
  beforeEach(() => {
    jest.clearAllMocks()
    console.error = jest.fn()
  })
  
  afterEach(() => {
    console.error = originalError
  })

  it('calls onConnected when wallet connects successfully', async () => {
    const onConnected = jest.fn()
    render(<WalletConnect onConnected={onConnected} />)
    
    const connectButton = screen.getByText(/Connect Wallet/i)
    await simulateSuccessfulConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(onConnected).toHaveBeenCalledWith('stars1mock...')
      expect(toast.success).toHaveBeenCalledWith('Wallet connected!', expect.any(Object))
    }, { timeout: 3000 })
  })

  it('shows error message on connection failure', async () => {
    const onConnected = jest.fn()
    render(<WalletConnect onConnected={onConnected} />)
    
    const connectButton = screen.getByText(/Connect Wallet/i)
    simulateFailedConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(console.error).toHaveBeenCalledWith('Error connecting wallet:', expect.any(Error))
      expect(toast.error).toHaveBeenCalledWith('Connection failed', expect.any(Object))
      expect(onConnected).not.toHaveBeenCalled()
    }, { timeout: 3000 })
  })
}) 