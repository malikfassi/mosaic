import React from 'react'
import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'
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
    expect(button).not.toBeDisabled()
    
    // Loading state
    rerender(<WalletConnect onConnected={() => {}} connecting={true} />)
    expect(button).toBeDisabled()
    expect(screen.getByRole('button')).toHaveClass('disabled:opacity-50')
  })
}) 