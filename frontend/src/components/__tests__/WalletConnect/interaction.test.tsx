import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import '@testing-library/jest-dom'
import WalletConnect from '../../WalletConnect'

describe('WalletConnect Interactions', () => {
  it('calls onConnected when wallet connects successfully', async () => {
    const onConnected = jest.fn()
    render(<WalletConnect onConnected={onConnected} />)
    
    const connectButton = screen.getByText(/Connect Wallet/i)
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(onConnected).toHaveBeenCalledWith('stars1mock...')
    })
  })

  it('shows error message on connection failure', async () => {
    window.keplr.enable.mockRejectedValueOnce(new Error('Connection failed'))
    
    render(<WalletConnect onConnected={() => {}} />)
    const connectButton = screen.getByText(/Connect Wallet/i)
    
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/Error connecting wallet/i)).toBeInTheDocument()
    })
  })
}) 