import { render, screen } from '@testing-library/react'
import WalletConnect from '../../WalletConnect'

describe('WalletConnect Rendering', () => {
  it('renders connect button when not connected', () => {
    render(<WalletConnect onConnected={() => {}} />)
    expect(screen.getByText(/Connect Wallet/i)).toBeInTheDocument()
  })

  it('shows loading state while connecting', () => {
    const { rerender } = render(<WalletConnect onConnected={() => {}} />)
    
    // Initial state
    const button = screen.getByRole('button')
    expect(button).not.toHaveAttribute('disabled')
    expect(button).not.toHaveClass('disabled:opacity-50')
    
    // Loading state
    rerender(<WalletConnect onConnected={() => {}} connecting={true} />)
    expect(button).toHaveAttribute('disabled')
    expect(button).toHaveClass('disabled:opacity-50')
  })
}) 