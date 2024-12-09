import { SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate';
import { GasPrice } from '@cosmjs/stargate';
import { useState, useCallback, useEffect } from 'react';
import { STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';

interface PixelColor {
  x: number;
  y: number;
  color: string;
  timestamp: number;
}

interface PixelHistory {
  x: number;
  y: number;
  colors: {
    color: string;
    timestamp: number;
    owner: string;
  }[];
}

interface TransactionStatus {
  isLoading: boolean;
  error: ContractError | null;
  hash?: string;
  success?: boolean;
}

type TransactionType = 'color' | 'connect';

interface ContractError extends Error {
  code?: number;
  message: string;
  type?: TransactionType;
  details?: string;
  cause?: unknown;
}

interface ContractHookResult {
  isConnected: boolean;
  isInitialized: boolean;
  address: string;
  error: ContractError | null;
  chainId: string;
  isLoading: boolean;
  transactionStatus: Record<TransactionType, TransactionStatus>;
  connect: () => Promise<void>;
  disconnect: () => void;
  setPixelColor: (x: number, y: number, color: string) => Promise<ExecuteResult>;
  getPixelColor: (x: number, y: number) => Promise<PixelColor>;
  getPixelHistory: (x: number, y: number) => Promise<PixelHistory>;
  getCanvas: () => Promise<PixelColor[]>;
  estimateGas: (type: TransactionType, params: any) => Promise<number>;
  balance: string;
}

const CHAIN_ID = process.env.NEXT_PUBLIC_STARGAZE_CHAIN_ID || 'elgafar-1';
const RPC_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com';
const REST_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_REST || 'https://rest.elgafar-1.stargaze-apis.com';
const COLORING_CONTRACT = process.env.NEXT_PUBLIC_COLORING_CONTRACT;
const GAS_PRICE = GasPrice.fromString('0.025ustars');

// Base gas costs
const BASE_GAS = 100_000;
const COLOR_GAS = 150_000;
const GAS_BUFFER = 1.3; // 30% buffer

const initialTransactionStatus = {
  isLoading: false,
  error: null,
};

export function useContract(): ContractHookResult {
  const [client, setClient] = useState<SigningCosmWasmClient | null>(null);
  const [address, setAddress] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<ContractError | null>(null);
  const [isInitialized, setIsInitialized] = useState(false);
  const [balance, setBalance] = useState('0');
  const [transactionStatus, setTransactionStatus] = useState<Record<TransactionType, TransactionStatus>>({
    color: initialTransactionStatus,
    connect: initialTransactionStatus,
  });

  // Auto-reconnect on mount if previously connected
  useEffect(() => {
    const lastAddress = localStorage.getItem('lastConnectedAddress');
    if (lastAddress) {
      connect().catch(console.error);
    }
  }, []);

  // Listen for Keplr account changes
  useEffect(() => {
    if (typeof window === 'undefined') return;

    const handleAccountsChanged = () => {
      if (isConnected) {
        connect().catch(console.error);
      }
    };

    window.addEventListener('keplr_keystorechange', handleAccountsChanged);
    return () => {
      window.removeEventListener('keplr_keystorechange', handleAccountsChanged);
    };
  }, [isConnected]);

  // Fetch balance periodically when connected
  useEffect(() => {
    if (!address || !isConnected) return;

    const fetchBalance = async () => {
      try {
        const response = await fetch(`${REST_ENDPOINT}/cosmos/bank/v1beta1/balances/${address}`);
        const data = await response.json();
        const ustarsBalance = data.balances?.find((b: any) => b.denom === 'ustars')?.amount || '0';
        const starsBalance = (parseInt(ustarsBalance) / 1_000_000).toFixed(2);
        setBalance(starsBalance);
      } catch (error) {
        console.error('Failed to fetch balance:', error);
      }
    };

    fetchBalance();
    const interval = setInterval(fetchBalance, 10000); // Update every 10 seconds
    return () => clearInterval(interval);
  }, [address, isConnected]);

  const updateTransactionStatus = (
    type: TransactionType,
    update: Partial<TransactionStatus>
  ) => {
    setTransactionStatus(prev => ({
      ...prev,
      [type]: { ...prev[type], ...update },
    }));
  };

  const connect = async (): Promise<void> => {
    if (isLoading) return;
    updateTransactionStatus('connect', { isLoading: true, error: null });
    
    try {
      if (!window.keplr) {
        const error = new Error('Keplr wallet not found') as ContractError;
        error.name = 'KeplrError';
        error.type = 'connect';
        error.details = 'Please install Keplr extension from https://www.keplr.app';
        throw error;
      }

      // Get chain info with environment variables
      const chainInfo = {
        ...getStargazeChainInfo(),
        chainId: CHAIN_ID,
        rpc: RPC_ENDPOINT,
        rest: REST_ENDPOINT,
      };

      try {
        await window.keplr.experimentalSuggestChain(chainInfo);
      } catch (err) {
        const error = new Error('Failed to suggest chain to Keplr') as ContractError;
        error.name = 'ChainError';
        error.type = 'connect';
        error.details = 'Chain configuration error';
        error.cause = err;
        throw error;
      }

      try {
        await window.keplr.enable(CHAIN_ID);
      } catch (err) {
        const error = new Error('Failed to enable Keplr for chain') as ContractError;
        error.name = 'EnableError';
        error.type = 'connect';
        error.details = `Chain ID: ${CHAIN_ID}`;
        error.cause = err;
        throw error;
      }

      let offlineSigner;
      try {
        offlineSigner = window.keplr.getOfflineSigner(CHAIN_ID);
      } catch (err) {
        const error = new Error('Failed to get offline signer') as ContractError;
        error.name = 'SignerError';
        error.type = 'connect';
        error.details = 'Could not get signer from Keplr';
        error.cause = err;
        throw error;
      }

      let newClient;
      try {
        newClient = await SigningCosmWasmClient.connectWithSigner(
          RPC_ENDPOINT,
          offlineSigner,
          { gasPrice: GAS_PRICE }
        );
      } catch (err) {
        const error = new Error('Failed to create signing client') as ContractError;
        error.name = 'ClientError';
        error.type = 'connect';
        error.details = `RPC Endpoint: ${RPC_ENDPOINT}`;
        error.cause = err;
        throw error;
      }

      let accounts;
      try {
        accounts = await offlineSigner.getAccounts();
      } catch (err) {
        const error = new Error('Failed to get accounts') as ContractError;
        error.name = 'AccountError';
        error.type = 'connect';
        error.details = 'Could not get accounts from Keplr';
        error.cause = err;
        throw error;
      }

      if (accounts.length === 0) {
        const error = new Error('No accounts found') as ContractError;
        error.name = 'AccountError';
        error.type = 'connect';
        error.details = 'Please add an account to Keplr';
        throw error;
      }

      const newAddress = accounts[0].address;
      
      setClient(newClient);
      setAddress(newAddress);
      setIsConnected(true);
      setIsInitialized(true);
      localStorage.setItem('lastConnectedAddress', newAddress);
      updateTransactionStatus('connect', { success: true });
    } catch (err) {
      const contractError = err as ContractError;
      if (!contractError.name) {
        contractError.name = 'ContractError';
      }

      console.error('Connection error:', {
        message: contractError.message,
        type: contractError.type,
        details: contractError.details,
        cause: contractError.cause
      });

      setError(contractError);
      setIsConnected(false);
      setClient(null);
      setAddress('');
      setIsInitialized(false);
      localStorage.removeItem('lastConnectedAddress');
      updateTransactionStatus('connect', { error: contractError });
      throw contractError;
    } finally {
      updateTransactionStatus('connect', { isLoading: false });
    }
  };

  const disconnect = useCallback(() => {
    setClient(null);
    setAddress('');
    setIsConnected(false);
    setError(null);
    setIsInitialized(false);
    setBalance('0');
    localStorage.removeItem('lastConnectedAddress');
  }, []);

  const setPixelColor = async (x: number, y: number, color: string): Promise<ExecuteResult> => {
    if (!client || !address) throw new Error('Not connected');
    if (!COLORING_CONTRACT) throw new Error('Coloring contract not configured');
    updateTransactionStatus('color', { isLoading: true, error: null });

    try {
      const result = await client.execute(
        address,
        COLORING_CONTRACT,
        {
          set_pixel_color: { x, y, color }
        },
        'auto',
        undefined,
        [{ denom: 'ustars', amount: '500000' }]
      );
      updateTransactionStatus('color', { success: true, hash: result.transactionHash });
      return result;
    } catch (err) {
      const colorError = err as ContractError;
      colorError.name = 'ColorError';
      colorError.type = 'color';
      updateTransactionStatus('color', { error: colorError });
      throw colorError;
    } finally {
      updateTransactionStatus('color', { isLoading: false });
    }
  };

  const getPixelColor = async (x: number, y: number): Promise<PixelColor> => {
    if (!client) throw new Error('Client not initialized');
    if (!COLORING_CONTRACT) throw new Error('Coloring contract not configured');

    const result = await client.queryContractSmart(COLORING_CONTRACT, {
      get_pixel_color: { x, y }
    });

    return {
      x,
      y,
      color: result.color,
      timestamp: result.timestamp,
    };
  };

  const getPixelHistory = async (x: number, y: number): Promise<PixelHistory> => {
    if (!client) throw new Error('Client not initialized');
    if (!COLORING_CONTRACT) throw new Error('Coloring contract not configured');

    const result = await client.queryContractSmart(COLORING_CONTRACT, {
      get_pixel_history: { x, y }
    });

    return {
      x,
      y,
      colors: result.colors,
    };
  };

  const getCanvas = async (): Promise<PixelColor[]> => {
    if (!client) throw new Error('Client not initialized');
    if (!COLORING_CONTRACT) throw new Error('Coloring contract not configured');

    const result = await client.queryContractSmart(COLORING_CONTRACT, {
      get_canvas: {}
    });

    return result.pixels;
  };

  const estimateGas = async (type: TransactionType, params: any): Promise<number> => {
    if (!client || !address) throw new Error('Not connected');
    if (!COLORING_CONTRACT) throw new Error('Coloring contract not configured');

    try {
      const msg = {
        set_pixel_color: {
          x: params.x,
          y: params.y,
          color: params.color
        }
      };

      const estimate = await client.simulate(
        address,
        [
          {
            typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
            value: {
              sender: address,
              contract: COLORING_CONTRACT,
              msg: Buffer.from(JSON.stringify(msg)).toString('base64'),
              funds: []
            }
          }
        ],
        ''
      );

      return Math.ceil(estimate * GAS_BUFFER);
    } catch (error) {
      console.error('Gas estimation failed:', error);
      return Math.ceil(COLOR_GAS * GAS_BUFFER);
    }
  };

  return {
    isConnected,
    isInitialized,
    address,
    error,
    chainId: CHAIN_ID,
    isLoading,
    transactionStatus,
    connect,
    disconnect,
    setPixelColor,
    getPixelColor,
    getPixelHistory,
    getCanvas,
    estimateGas,
    balance,
  };
} 