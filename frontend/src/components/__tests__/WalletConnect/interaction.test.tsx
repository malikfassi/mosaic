import { render, screen, fireEvent, waitFor } from '@testing-library/react'
import WalletConnect from '../../WalletConnect'
import { simulateSuccessfulConnection, simulateFailedConnection } from './setup'

describe('WalletConnect Interactions', () => {
  it('calls onConnected when wallet connects successfully', async () => {
    const onConnected = jest.fn()
    render(<WalletConnect onConnected={onConnected} />)
    
    const connectButton = screen.getByText(/Connect Wallet/i)
    await simulateSuccessfulConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(onConnected).toHaveBeenCalledWith('stars1mock...')
    }, { timeout: 3000 })
  })

  it('shows error message on connection failure', async () => {
    const onConnected = jest.fn()
    render(<WalletConnect onConnected={onConnected} />)
    
    const connectButton = screen.getByText(/Connect Wallet/i)
    simulateFailedConnection()
    fireEvent.click(connectButton)
    
    await waitFor(() => {
      expect(screen.getByText(/Failed to connect wallet/i)).toBeInTheDocument()
    }, { timeout: 3000 })
  })
}) 