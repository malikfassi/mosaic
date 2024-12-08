import { Window as KeplrWindow } from "@keplr-wallet/types";
import { OfflineSigner } from "@cosmjs/proto-signing";
import { ChainInfo } from "@keplr-wallet/types";

declare global {
  interface Window extends KeplrWindow {
    keplr: {
      enable: (chainId: string) => Promise<void>;
      getKey: (chainId: string) => Promise<{ bech32Address: string; pubKey: Uint8Array }>;
      experimentalSuggestChain: (chainInfo: ChainInfo) => Promise<void>;
      getOfflineSigner: (chainId: string) => OfflineSigner;
      getOfflineSignerOnlyAmino: (chainId: string) => OfflineSigner;
      getOfflineSignerAuto: (chainId: string) => Promise<OfflineSigner>;
      signArbitrary: (chainId: string, signer: string, data: string) => Promise<{
        signature: Uint8Array;
        pub_key: { type: string; value: string };
      }>;
    };
  }
}

export {} 