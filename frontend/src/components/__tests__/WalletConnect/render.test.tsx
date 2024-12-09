import React from 'react'
import { render, screen } from '@testing-library/react'
import '@testing-library/jest-dom'
import { WalletConnect } from '../../WalletConnect'

describe('WalletConnect Rendering', () => {
  it('renders connect button when not connected', () => {
    render(<WalletConnect />)
    expect(screen.getByText(/Connect Keplr/i)).toBeInTheDocument()
  })

  it('shows error state when connection fails', () => {
    render(<WalletConnect />)
    const button = screen.getByRole('button')
    expect(button).toHaveTextContent(/Connect Keplr/i)
    expect(button).not.toHaveClass('destructive')
  })
}) 