import React from 'react'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import '@testing-library/jest-dom'
import { WalletConnect } from '../../WalletConnect'
import { simulateSuccessfulConnection, simulateFailedConnection } from './setup'

// Mock useToast
jest.mock('@/components/ui/use-toast', () => ({
  useToast: () => ({
    toast: jest.fn()
  })
}))

describe('WalletConnect Interactions', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('connects wallet successfully', async () => {
    render(<WalletConnect />)
    
    const connectButton = screen.getByText(/Connect Keplr/i)
    await simulateSuccessfulConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/Disconnect/i)).toBeInTheDocument()
      expect(screen.getByText(/stars1mock.../i)).toBeInTheDocument()
    })
  })

  it('shows retry button on connection failure', async () => {
    render(<WalletConnect />)
    
    const connectButton = screen.getByText(/Connect Keplr/i)
    simulateFailedConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/Retry Connection/i)).toBeInTheDocument()
    })
  })
}) 