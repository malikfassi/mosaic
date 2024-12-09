import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';
import { useCallback, useEffect, useState } from 'react';
import { STARGAZE_RPC, STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';

// Custom error types for better error handling
interface KeplrError extends Error {
  code?: number;
  details?: string;
}

interface TransactionStatus {
  isLoading: boolean;
  error: Error | null;
}

interface ContractState {
  client: SigningCosmWasmClient | null;
  address: string;
  isConnected: boolean;
  isInitialized: boolean;
  error: Error | null;
  transactionStatus: {
    connect: TransactionStatus;
    mint: TransactionStatus;
    colorChange: TransactionStatus;
  };
}

const initialTransactionStatus: TransactionStatus = {
  isLoading: false,
  error: null,
};

const initialState: ContractState = {
  client: null,
  address: '',
  isConnected: false,
  isInitialized: false,
  error: null,
  transactionStatus: {
    connect: initialTransactionStatus,
    mint: initialTransactionStatus,
    colorChange: initialTransactionStatus,
  },
};

export function useContract() {
  const [state, setState] = useState<ContractState>(initialState);

  const updateTransactionStatus = useCallback((
    transaction: keyof ContractState['transactionStatus'],
    updates: Partial<TransactionStatus>
  ) => {
    setState(prev => ({
      ...prev,
      transactionStatus: {
        ...prev.transactionStatus,
        [transaction]: {
          ...prev.transactionStatus[transaction],
          ...updates,
        },
      },
    }));
  }, []);

  const connect = useCallback(async () => {
    try {
      updateTransactionStatus('connect', { isLoading: true, error: null });

      // Check if Keplr is installed
      if (!window.keplr) {
        throw new Error('Please install Keplr extension');
      }

      // Get chain info and suggest it to Keplr
      const chainInfo = getStargazeChainInfo();
      await window.keplr.experimentalSuggestChain(chainInfo);

      // Enable Keplr for the Stargaze chain
      await window.keplr.enable(STARGAZE_CHAIN_ID);

      // Get the offlineSigner for this chain
      const offlineSigner = window.keplr.getOfflineSigner(STARGAZE_CHAIN_ID);

      // Get the user's Stargaze address
      const accounts = await offlineSigner.getAccounts();
      const address = accounts[0].address;

      // Create the signing client
      const client = await SigningCosmWasmClient.connectWithSigner(
        STARGAZE_RPC,
        offlineSigner,
        {
          gasPrice: GasPrice.fromString('0.025ustars'),
        }
      );

      setState(prev => ({
        ...prev,
        client,
        address,
        isConnected: true,
        isInitialized: true,
        error: null,
      }));

      updateTransactionStatus('connect', { isLoading: false });
    } catch (error) {
      const keplrError = error as KeplrError;
      console.error('Connection error:', {
        message: keplrError.message,
        code: keplrError.code,
        details: keplrError.details,
      });

      setState(prev => ({
        ...prev,
        error: keplrError,
        isConnected: false,
      }));

      updateTransactionStatus('connect', {
        isLoading: false,
        error: keplrError,
      });

      throw error;
    }
  }, [updateTransactionStatus]);

  const disconnect = useCallback(() => {
    setState(initialState);
  }, []);

  // Auto-connect if Keplr is available
  useEffect(() => {
    if (window.keplr && !state.isConnected && !state.error) {
      connect().catch(console.error);
    }
  }, [connect, state.isConnected, state.error]);

  // Reconnect on chain change
  useEffect(() => {
    if (!window.keplr) return;

    const handleChainChanged = () => {
      if (state.isConnected) {
        connect().catch(console.error);
      }
    };

    window.addEventListener('keplr_keystorechange', handleChainChanged);

    return () => {
      window.removeEventListener('keplr_keystorechange', handleChainChanged);
    };
  }, [connect, state.isConnected]);

  return {
    ...state,
    connect,
    disconnect,
  };
} 