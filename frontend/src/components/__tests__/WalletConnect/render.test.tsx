import { render, screen } from '@testing-library/react'
import WalletConnect from '../../WalletConnect'

describe('WalletConnect Rendering', () => {
  it('renders connect button when not connected', () => {
    render(<WalletConnect onConnected={() => {}} />)
    expect(screen.getByText(/Connect Wallet/i)).toBeInTheDocument()
  })

  it('shows loading state while connecting', () => {
    render(<WalletConnect onConnected={() => {}} />)
    const button = screen.getByRole('button')
    expect(button).toHaveAttribute('disabled', '')
    expect(button).toHaveClass('disabled:opacity-50')
  })
}) 