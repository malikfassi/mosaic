import React from 'react'
import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import '@testing-library/jest-dom'
import { WalletConnect } from '../../WalletConnect'
import { simulateSuccessfulConnection, simulateFailedConnection } from './setup'
import { act } from '@testing-library/react'

// Mock useToast
jest.mock('@/components/ui/use-toast', () => ({
  useToast: () => ({
    toast: jest.fn()
  })
}))

// Helper function to wait for state updates
const waitForStateUpdate = async () => {
  await act(async () => {
    await new Promise(resolve => setTimeout(resolve, 100))
  })
}

describe('WalletConnect Interactions', () => {
  beforeEach(() => {
    jest.clearAllMocks()
  })

  it('connects wallet successfully', async () => {
    render(<WalletConnect />)
    
    simulateSuccessfulConnection()
    const connectButton = screen.getByText(/Connect Keplr/i)
    
    await act(async () => {
      fireEvent.click(connectButton)
      await waitForStateUpdate()
    })
    
    await waitFor(() => {
      expect(screen.getByText(/Disconnect/i)).toBeInTheDocument()
      expect(screen.getByText(/stars1mock.../i)).toBeInTheDocument()
    }, { timeout: 2000 })
  })

  it('shows retry button on connection failure', async () => {
    render(<WalletConnect />)
    
    simulateFailedConnection()
    const connectButton = screen.getByText(/Connect Keplr/i)
    
    await act(async () => {
      fireEvent.click(connectButton)
      await waitForStateUpdate()
    })
    
    await waitFor(() => {
      expect(screen.getByText(/Retry Connection/i)).toBeInTheDocument()
    }, { timeout: 2000 })
  })
}) 