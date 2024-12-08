// Learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom'

// Mock WebSocket
class MockWebSocket {
  constructor(url) {
    this.url = url;
    this.readyState = WebSocket.CONNECTING;
    setTimeout(() => {
      this.readyState = WebSocket.OPEN;
      if (this.onopen) this.onopen();
    }, 0);
  }

  send(data) {
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

global.WebSocket = MockWebSocket;

// Mock window.keplr
const mockKeplr = {
  enable: jest.fn().mockResolvedValue(true),
  getKey: jest.fn().mockResolvedValue({
    bech32Address: 'stars1mock...',
    pubKey: new Uint8Array([1, 2, 3]),
  }),
};

Object.defineProperty(window, 'keplr', {
  value: mockKeplr,
});

// Mock ResizeObserver
global.ResizeObserver = class ResizeObserver {
  observe() {}
  unobserve() {}
  disconnect() {}
}; 