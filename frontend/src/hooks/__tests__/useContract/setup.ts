import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { Window as KeplrWindow } from "@keplr-wallet/types"
import { renderHook } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import React, { ReactNode } from 'react'

declare global {
  interface Window extends KeplrWindow {}
}

// Mock SigningCosmWasmClient
const mockSigningClient = {
  connect: jest.fn(),
  disconnect: jest.fn(),
  simulate: jest.fn().mockResolvedValue({ gasUsed: 100000 }),
  execute: jest.fn().mockResolvedValue({
    transactionHash: '0x123...',
    code: 0,
    rawLog: 'success',
  }),
} as const

jest.mock('@cosmjs/cosmwasm-stargate', () => ({
  SigningCosmWasmClient: {
    connectWithSigner: jest.fn().mockResolvedValue(mockSigningClient),
  },
}))

// Test setup utilities
export const createTestQueryClient = () => 
  new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        cacheTime: 0,
      },
    },
  })

type WrapperProps = {
  children: ReactNode;
};

export const createWrapper = () => {
  const queryClient = createTestQueryClient();
  const Wrapper = ({ children }: WrapperProps) => (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
  return Wrapper;
};

export const mockContractAddress = "stars14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf4hyy"

export const setupHookTest = () => {
  const queryClient = createTestQueryClient()
  const wrapper = createWrapper()
  
  return {
    queryClient,
    wrapper,
    mockSigningClient,
    renderTestHook: <TResult,>(hook: () => TResult) => renderHook(hook, { wrapper }),
  }
} 