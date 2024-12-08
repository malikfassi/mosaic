import { SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate';
import { useState, useCallback } from 'react';
import { STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';

interface ContractHookResult {
  isConnected: boolean;
  address: string;
  error: Error | null;
  chainId: string;
  connect: () => Promise<void>;
  disconnect: () => void;
  buyPixel: (x: number, y: number) => Promise<ExecuteResult>;
  setPixelColor: (x: number, y: number, color: string) => Promise<ExecuteResult>;
  getPixel: (x: number, y: number) => Promise<any>;
  getCanvas: () => Promise<any>;
}

export function useContract(): ContractHookResult {
  const [client, setClient] = useState<SigningCosmWasmClient | null>(null);
  const [address, setAddress] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const connect = async () => {
    try {
      // Suggest chain to Keplr
      await window.keplr.experimentalSuggestChain(getStargazeChainInfo());
      await window.keplr.enable(STARGAZE_CHAIN_ID);

      const offlineSigner = window.keplr.getOfflineSigner(STARGAZE_CHAIN_ID);
      const client = await SigningCosmWasmClient.connectWithSigner(
        getStargazeChainInfo().rpc,
        offlineSigner
      );

      const accounts = await offlineSigner.getAccounts();
      if (accounts.length > 0) {
        setAddress(accounts[0].address);
        setClient(client);
        setIsConnected(true);
        setError(null);
      }
    } catch (err) {
      setError(err instanceof Error ? err : new Error('Failed to connect'));
      setIsConnected(false);
    }
  };

  const disconnect = useCallback(() => {
    setClient(null);
    setAddress('');
    setIsConnected(false);
    setError(null);
  }, []);

  return {
    isConnected,
    address,
    error,
    chainId: STARGAZE_CHAIN_ID,
    connect,
    disconnect,
    buyPixel: async (x: number, y: number) => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {} as ExecuteResult;
    },
    setPixelColor: async (x: number, y: number, color: string) => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {} as ExecuteResult;
    },
    getPixel: async (x: number, y: number) => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {};
    },
    getCanvas: async () => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {};
    }
  };
} 