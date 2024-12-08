import '@testing-library/jest-dom'
import { OfflineSigner } from '@cosmjs/proto-signing'
import { ChainInfo } from '@keplr-wallet/types'

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
      experimentalSuggestChain: jest.Mock<Promise<void>>
      getOfflineSigner: jest.Mock<OfflineSigner>
      getOfflineSignerOnlyAmino: jest.Mock<OfflineSigner>
      getOfflineSignerAuto: jest.Mock<Promise<OfflineSigner>>
      signArbitrary: jest.Mock<Promise<{
        signature: Uint8Array
        pub_key: {
          type: string
          value: string
        }
      }>>
    }
  }
}

export {} 