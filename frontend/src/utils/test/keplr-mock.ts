import { jest } from '@jest/globals';
import { Keplr, Key, KeplrSignOptions, DirectSignResponse } from '@keplr-wallet/types';
import Long from 'long';

type EnableFn = (chainIds: string | string[]) => Promise<void>;
type GetKeyFn = (chainId: string) => Promise<Key>;
type SignDirectFn = (
  chainId: string,
  signer: string,
  signDoc: {
    bodyBytes?: Uint8Array | null;
    authInfoBytes?: Uint8Array | null;
    chainId?: string | null;
    accountNumber?: Long | null;
  },
  signOptions?: KeplrSignOptions
) => Promise<DirectSignResponse>;

export const mockKeplr: jest.Mocked<Partial<Keplr>> = {
  enable: jest.fn<EnableFn>().mockImplementation(async () => Promise.resolve()),
  getKey: jest.fn<GetKeyFn>().mockImplementation(async () => ({
    name: 'mock-key',
    algo: 'secp256k1',
    pubKey: new Uint8Array([1, 2, 3]),
    address: new Uint8Array([1, 2, 3]),
    bech32Address: 'stars1mock...',
    isNanoLedger: false,
    isKeystone: false,
    ethereumHexAddress: '0x1234567890abcdef'
  })),
  signDirect: jest.fn<SignDirectFn>().mockImplementation(async (chainId, _signer, _signDoc) => ({
    signed: {
      bodyBytes: new Uint8Array(),
      authInfoBytes: new Uint8Array(),
      chainId: chainId,
      accountNumber: Long.fromNumber(0)
    },
    signature: {
      pub_key: {
        type: "tendermint/PubKeySecp256k1",
        value: "test"
      },
      signature: "test"
    }
  }))
}; 