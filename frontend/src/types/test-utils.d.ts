import '@testing-library/jest-dom'

declare global {
  namespace jest {
    interface Matchers<R> {
      toBeInTheDocument(): R
      toHaveStyle(style: Record<string, any>): R
    }
  }

  interface Window {
    keplr: {
      enable: jest.Mock<Promise<void>>
      getKey: jest.Mock<Promise<{
        bech32Address: string
        pubKey: Uint8Array
      }>>
    }
  }
}

export {} 