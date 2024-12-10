import '@testing-library/jest-dom';
import { Window as KeplrWindow } from "@keplr-wallet/types";
import { TextEncoder } from 'util';

// Extend window with Keplr types
declare global {
  interface Window extends KeplrWindow {
    // Add a dummy property to satisfy ESLint
    _keplrInitialized?: boolean;
  }
}

// Mock TextEncoder as it's not available in jsdom
global.TextEncoder = TextEncoder;

// Create a basic Keplr mock with common functions
const mockKeplr = {
  enable: jest.fn().mockResolvedValue(undefined),
  getKey: jest.fn().mockResolvedValue({
    name: 'Test Wallet',
    algo: 'secp256k1',
    pubKey: new Uint8Array(),
    address: new Uint8Array(),
    bech32Address: 'stars1example...',
    isNanoLedger: false,
  }),
  getOfflineSigner: jest.fn().mockReturnValue({
    getAccounts: jest.fn().mockResolvedValue([{
      address: 'stars1example...',
      algo: 'secp256k1',
      pubkey: new Uint8Array(),
    }]),
    signDirect: jest.fn(),
  }),
  signArbitrary: jest.fn(),
  experimentalSuggestChain: jest.fn().mockResolvedValue(undefined),
};

// Mock window.keplr following Keplr docs
Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
  writable: true,
  configurable: true,
});

// Set the initialization flag
window._keplrInitialized = true;

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

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