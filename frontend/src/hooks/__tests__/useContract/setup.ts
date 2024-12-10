import '@testing-library/jest-dom'
import { jest } from '@jest/globals'
import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate'
import { Window as KeplrWindow } from "@keplr-wallet/types"
import { renderHook } from '@testing-library/react'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import React, { ReactNode } from 'react'
import { OfflineSigner, AccountData } from '@cosmjs/proto-signing'
import { TextEncoder } from 'util'

declare global {
  // This extends the global Window interface with Keplr types
  interface Window extends KeplrWindow {
    keplr: KeplrWindow['keplr'];
    _keplrInitialized?: boolean;
  }
}

// Mock TextEncoder as it's not available in jsdom
global.TextEncoder = TextEncoder;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// Mock contract address
export const mockContractAddress = "stars14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sgf4hyy"

// Mock account data
const mockAccount: AccountData = {
  address: mockContractAddress,
  algo: 'secp256k1',
  pubkey: new Uint8Array(),
}

// Mock Keplr's offline signer with proper getAccounts implementation
const mockOfflineSigner: OfflineSigner = {
  getAccounts: jest.fn().mockResolvedValue([mockAccount]),
  signDirect: jest.fn(),
}

// Create a basic Keplr mock with common functions
const mockKeplr = {
  enable: jest.fn().mockResolvedValue(undefined),
  getKey: jest.fn().mockResolvedValue({
    name: 'Test Wallet',
    algo: 'secp256k1',
    pubKey: new Uint8Array(),
    address: new Uint8Array(),
    bech32Address: mockContractAddress,
    isNanoLedger: false,
  }),
  getOfflineSigner: jest.fn().mockReturnValue(mockOfflineSigner),
  signArbitrary: jest.fn(),
  experimentalSuggestChain: jest.fn().mockResolvedValue(undefined),
};

// Mock window.keplr
Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
  writable: true,
  configurable: true,
});

// Set the initialization flag
window._keplrInitialized = true;

type MockSigningClientType = Partial<SigningCosmWasmClient> & {
  connect: jest.Mock;
  disconnect: jest.Mock;
  simulate: jest.Mock;
  execute: jest.Mock;
}

// Mock SigningCosmWasmClient
const mockSigningClient: MockSigningClientType = {
  connect: jest.fn(),
  disconnect: jest.fn(),
  simulate: jest.fn().mockResolvedValue(100000),
  execute: jest.fn().mockResolvedValue({
    transactionHash: '0x123...',
    code: 0,
    rawLog: 'success',
  }),
}

// Mock SigningCosmWasmClient and its static methods
const mockConnectWithSigner = jest.fn().mockResolvedValue(mockSigningClient);
jest.mock('@cosmjs/cosmwasm-stargate', () => ({
  SigningCosmWasmClient: {
    connectWithSigner: mockConnectWithSigner,
  },
}));

// Test setup utilities
export const createTestQueryClient = () => 
  new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        staleTime: 0,
      },
    },
  })

type WrapperProps = {
  children: ReactNode;
};

export const createWrapper = () => {
  const queryClient = createTestQueryClient();
  return function TestWrapper({ children }: WrapperProps) {
    return React.createElement(QueryClientProvider, { client: queryClient }, children);
  };
};

// Mock Keplr wallet functions
export const simulateSuccessfulConnection = () => {
  if (window.keplr) {
    (window.keplr.enable as jest.Mock).mockResolvedValueOnce(undefined);
    (window.keplr.experimentalSuggestChain as jest.Mock).mockResolvedValueOnce(undefined);
    (window.keplr.getOfflineSigner as jest.Mock).mockReturnValue(mockOfflineSigner);
  }
};

export const simulateFailedConnection = () => {
  const error = new Error('Connection failed');
  if (window.keplr) {
    (window.keplr.enable as jest.Mock).mockRejectedValueOnce(error);
    (window.keplr.experimentalSuggestChain as jest.Mock).mockRejectedValueOnce(error);
    // Mock SigningCosmWasmClient to also fail
    mockConnectWithSigner.mockRejectedValueOnce(error);
  }
  return error;
};

export const setupHookTest = () => {
  const queryClient = createTestQueryClient()
  const wrapper = createWrapper()
  
  return {
    queryClient,
    wrapper,
    mockSigningClient,
    renderTestHook: <TResult>(hook: () => TResult) => renderHook(hook, { wrapper }),
  }
}

// Clear mocks after each test
afterEach(() => {
  // Reset all mock implementations
  if (window.keplr) {
    Object.values(window.keplr).forEach(mock => {
      if (typeof mock === 'function' && 'mockClear' in mock) {
        mock.mockClear();
      }
    });
  }
}); 