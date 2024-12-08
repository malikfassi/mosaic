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
  getPixel: (x: number, y: number) => Promise<unknown>;
  getCanvas: () => Promise<unknown>;
}

export function useContract(): ContractHookResult {
  const [client, setClient] = useState<SigningCosmWasmClient | null>(null);
  const [address, setAddress] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<Error | null>(null);

  const connect = async () => {
    try {
      if (!window.keplr) {
        throw new Error('Keplr wallet not found');
      }

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
      throw err;
    }
  };

  const disconnect = useCallback(() => {
    setClient(null);
    setAddress('');
    setIsConnected(false);
    setError(null);
  }, []);

  const getPixel = async (_x: number, _y: number) => {
    if (!client) throw new Error('Not connected');
    // Implementation
    return {};
  };

  return {
    isConnected,
    address,
    error,
    chainId: STARGAZE_CHAIN_ID,
    connect,
    disconnect,
    buyPixel: async (_x: number, _y: number) => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {} as ExecuteResult;
    },
    setPixelColor: async (_x: number, _y: number, _color: string) => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {} as ExecuteResult;
    },
    getPixel,
    getCanvas: async () => {
      if (!client) throw new Error('Not connected');
      // Implementation
      return {};
    }
  };
} 