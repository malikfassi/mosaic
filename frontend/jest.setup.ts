import '@testing-library/jest-dom'
import { jest } from '@jest/globals'

// Define WebSocket constants
const WebSocket = {
  CONNECTING: 0,
  OPEN: 1,
  CLOSING: 2,
  CLOSED: 3,
} as const;

// Mock WebSocket
class MockWebSocket {
  url: string;
  readyState: number;
  onopen?: () => void;
  onmessage?: (event: { data: any }) => void;
  onclose?: () => void;

  constructor(url: string) {
    this.url = url;
    this.readyState = WebSocket.CONNECTING;
    setTimeout(() => {
      this.readyState = WebSocket.OPEN;
      if (this.onopen) this.onopen();
    }, 0);
  }

  send(data: any) {
    if (this.onmessage) {
      // Echo back the data for testing purposes
      this.onmessage({ data });
    }
  }

  close() {
    this.readyState = WebSocket.CLOSED;
    if (this.onclose) this.onclose();
  }
}

// Set up global WebSocket mock
(global as any).WebSocket = MockWebSocket;
(global as any).WebSocket.CONNECTING = WebSocket.CONNECTING;
(global as any).WebSocket.OPEN = WebSocket.OPEN;
(global as any).WebSocket.CLOSING = WebSocket.CLOSING;
(global as any).WebSocket.CLOSED = WebSocket.CLOSED;

// Mock window.keplr
const mockKeplr = {
  enable: jest.fn<() => Promise<void>>().mockResolvedValue(),
  getKey: jest.fn<() => Promise<{ bech32Address: string; pubKey: Uint8Array }>>().mockResolvedValue({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  }),
};

Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
});

// Mock ResizeObserver
(global as any).ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}; 