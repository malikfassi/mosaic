import '@testing-library/jest-dom';
import { jest } from '@jest/globals';

// Define WebSocket constants
const WebSocket = {
  CONNECTING: 0,
  OPEN: 1,
  CLOSING: 2,
  CLOSED: 3,
};

// Mock WebSocket globally
global.WebSocket = class MockWebSocket {
  constructor() {
    this.readyState = WebSocket.CLOSED;
  }
  close() {}
  send() {}
};

// Mock window.keplr
const mockKeplr = {
  enable: jest.fn().mockResolvedValue(undefined),
  getKey: jest.fn().mockResolvedValue({
    name: 'mock-key',
    algo: 'secp256k1',
    pubKey: new Uint8Array([1, 2, 3]),
    address: new Uint8Array([1, 2, 3]),
    bech32Address: 'stars1mock...',
    isNanoLedger: false,
    isKeystone: false,
    ethereumHexAddress: '0x1234567890abcdef'
  }),
  experimentalSuggestChain: jest.fn().mockResolvedValue(undefined),
  getOfflineSigner: jest.fn().mockReturnValue({
    getAccounts: jest.fn().mockResolvedValue([{
      address: 'stars1mock...',
      pubkey: new Uint8Array([1, 2, 3]),
      algo: 'secp256k1'
    }]),
    signDirect: jest.fn().mockResolvedValue({
      signed: {
        bodyBytes: new Uint8Array(),
        authInfoBytes: new Uint8Array(),
        chainId: "test-chain",
        accountNumber: 0
      },
      signature: {
        pub_key: {
          type: "tendermint/PubKeySecp256k1",
          value: "test"
        },
        signature: "test"
      }
    })
  })
};

global.window = {
  ...global.window,
  keplr: mockKeplr,
};

// Mock console.error
console.error = jest.fn();
  