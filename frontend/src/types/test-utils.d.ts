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
      enable: jest.Mock<Promise<void>, [string | string[]]>
      getKey: jest.Mock<Promise<{
        bech32Address: string
        pubKey: Uint8Array
      }>, [string]>
      experimentalSuggestChain: jest.Mock<Promise<void>, [ChainInfo]>
      getOfflineSigner: jest.Mock<OfflineSigner, [string]>
      getOfflineSignerOnlyAmino: jest.Mock<OfflineSigner, [string]>
      getOfflineSignerAuto: jest.Mock<Promise<OfflineSigner>, [string]>
      signArbitrary: jest.Mock<Promise<{
        signature: Uint8Array
        pub_key: {
          type: string
          value: string
        }
      }>, [string, string, string]>
    }
  }
}

export {} 