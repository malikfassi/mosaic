import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import '@testing-library/jest-dom'
import WalletConnect from '../WalletConnect'

describe('WalletConnect', () => {
  it('renders connect button when not connected', () => {
    render(<WalletConnect />)
    expect(screen.getByText(/Connect Wallet/i)).toBeInTheDocument()
  })

  it('connects wallet when button is clicked', async () => {
    render(<WalletConnect />)
    const connectButton = screen.getByText(/Connect Wallet/i)
    
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(window.keplr.enable).toHaveBeenCalled()
    })
  })

  it('shows error message when wallet connection fails', async () => {
    // Mock keplr.enable to reject
    window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
    
    render(<WalletConnect />)
    const connectButton = screen.getByText(/Connect Wallet/i)
    
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/Error connecting wallet/i)).toBeInTheDocument()
    })
  })

  it('shows connected address when wallet is connected', async () => {
    render(<WalletConnect />)
    const connectButton = screen.getByText(/Connect Wallet/i)
    
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/stars1mock.../i)).toBeInTheDocument()
    })
  })
}) 