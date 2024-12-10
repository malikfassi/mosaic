import '@testing-library/jest-dom';
import { Window as KeplrWindow } from "@keplr-wallet/types";
import { TextEncoder } from 'util';

// Extend window with Keplr types
declare global {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface Window extends KeplrWindow {
    // Add any additional window properties here if needed
    foo?: string;
  }
}

// Mock TextEncoder as it's not available in jsdom
global.TextEncoder = TextEncoder;

// Create a comprehensive Keplr mock following Keplr docs
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
};

// Mock window.keplr following Keplr docs
Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
  writable: true,
  configurable: true,
});

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
};

// Clear mocks after each test
afterEach(() => {
  jest.clearAllMocks();
}); 