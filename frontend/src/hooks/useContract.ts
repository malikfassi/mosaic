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
  chainId: string;
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
  chainId: STARGAZE_CHAIN_ID,
  transactionStatus: {
    connect: initialTransactionStatus,
    mint: initialTransactionStatus,
    colorChange: initialTransactionStatus,
  },
};

const disconnectedState = (error: Error | null = null): ContractState => ({
  ...initialState,
  isInitialized: true,
  error,
  transactionStatus: {
    connect: { isLoading: false, error },
    mint: { isLoading: false, error: null },
    colorChange: { isLoading: false, error: null }
  }
});

export function useContract() {
  const [state, setState] = useState<ContractState>(initialState);

  const connect = useCallback(async () => {
    try {
      // Reset error state and set loading
      setState(prev => ({
        ...prev,
        isConnected: false,
        error: null,
        transactionStatus: {
          ...prev.transactionStatus,
          connect: { isLoading: true, error: null }
        }
      }));

      // Check if Keplr is installed
      if (!window.keplr) {
        const error = new Error('Please install Keplr extension');
        setState(disconnectedState(error));
        throw error;
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

      try {
        // Create the signing client
        const client = await SigningCosmWasmClient.connectWithSigner(
          STARGAZE_RPC,
          offlineSigner,
          {
            gasPrice: GasPrice.fromString('0.025ustars'),
          }
        );

        // Update state with successful connection
        setState(prev => ({
          ...prev,
          client,
          address,
          isConnected: true,
          isInitialized: true,
          error: null,
          transactionStatus: {
            ...prev.transactionStatus,
            connect: { isLoading: false, error: null }
          }
        }));
      } catch (error) {
        // Handle client connection error
        const keplrError = error as KeplrError;
        setState(prev => ({
          ...initialState,
          isInitialized: true,
          isConnected: false,
          error: keplrError,
          transactionStatus: {
            ...prev.transactionStatus,
            connect: { isLoading: false, error: keplrError }
          }
        }));
        throw keplrError;
      }
    } catch (error) {
      const keplrError = error as KeplrError;
      console.error('Connection error:', {
        message: keplrError.message,
        code: keplrError.code,
        details: keplrError.details,
      });

      // Use the fixed disconnectedState function
      setState(disconnectedState(keplrError));
      throw keplrError;
    }
  }, []);

  const disconnect = useCallback(() => {
    setState(disconnectedState(null));
  }, []);

  // Auto-connect if Keplr is available
  useEffect(() => {
    let mounted = true;

    if (window.keplr && !state.isConnected && !state.error) {
      connect().catch(error => {
        if (mounted) {
          console.error(error);
          setState(disconnectedState(error));
        }
      });
    }

    return () => {
      mounted = false;
    };
  }, [connect, state.isConnected, state.error]);

  // Reconnect on chain change
  useEffect(() => {
    if (!window.keplr) return;

    const handleChainChanged = () => {
      if (state.isConnected) {
        connect().catch(error => {
          console.error(error);
          setState(disconnectedState(error));
        });
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