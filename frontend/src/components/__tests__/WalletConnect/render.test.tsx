import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'
import WalletConnect from '../../WalletConnect'

describe('WalletConnect Rendering', () => {
  it('renders connect button when not connected', () => {
    render(<WalletConnect onConnected={() => {}} />)
    expect(screen.getByText(/Connect Wallet/i)).toBeInTheDocument()
  })

  it('shows connected address when wallet is connected', async () => {
    const mockAddress = 'stars1mock...'
    render(
      <WalletConnect 
        onConnected={() => {}} 
        initialAddress={mockAddress} 
      />
    )
    expect(screen.getByText(mockAddress)).toBeInTheDocument()
  })
}) 