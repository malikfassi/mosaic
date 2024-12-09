import { SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate';
import { GasPrice, Coin } from '@cosmjs/stargate';
import { useState, useCallback, useEffect } from 'react';
import { STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';

interface PixelData {
  x: number;
  y: number;
  color: string;
}

interface PixelInfo {
  owner: string;
  color: string;
}

interface ContractError extends Error {
  code?: number;
  message: string;
}

interface ContractHookResult {
  isConnected: boolean;
  address: string;
  error: ContractError | null;
  chainId: string;
  isLoading: boolean;
  connect: () => Promise<void>;
  disconnect: () => void;
  buyPixel: (x: number, y: number) => Promise<ExecuteResult>;
  buyPixels: (pixels: PixelData[]) => Promise<ExecuteResult>;
  setPixelColor: (x: number, y: number, color: string) => Promise<ExecuteResult>;
  getPixel: (x: number, y: number) => Promise<PixelInfo>;
  getCanvas: () => Promise<Array<[number, number, PixelInfo]>>;
  estimateGas: (pixels: PixelData[]) => Promise<number>;
}

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS;
const CHAIN_ID = process.env.NEXT_PUBLIC_STARGAZE_CHAIN_ID || 'elgafar-1';
const RPC_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com';
const REST_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_REST || 'https://rest.elgafar-1.stargaze-apis.com';
const GAS_PRICE = GasPrice.fromString('0.025ustars');

// Base gas costs
const BASE_GAS = 100_000;
const GAS_PER_PIXEL = 50_000;
const GAS_BUFFER = 1.5; // 50% buffer

const validateContractAddress = (address: string | undefined): string => {
  if (!address) {
    throw new Error('Contract address not configured');
  }
  if (!address.startsWith('stars')) {
    throw new Error('Invalid contract address format. Must start with "stars"');
  }
  return address;
};

export function useContract(): ContractHookResult {
  const [client, setClient] = useState<SigningCosmWasmClient | null>(null);
  const [address, setAddress] = useState('');
  const [isConnected, setIsConnected] = useState(false);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<ContractError | null>(null);

  // Auto-reconnect on mount if previously connected
  useEffect(() => {
    const lastAddress = localStorage.getItem('lastConnectedAddress');
    if (lastAddress && window.keplr) {
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

  const connect = async (): Promise<void> => {
    if (isLoading) return;
    setIsLoading(true);
    setError(null);
    
    try {
      if (!window.keplr) {
        throw new Error('Keplr wallet not found');
      }

      // Validate contract address early
      validateContractAddress(CONTRACT_ADDRESS);

      // Get chain info with environment variables
      const chainInfo = {
        ...getStargazeChainInfo(),
        chainId: CHAIN_ID,
        rpc: RPC_ENDPOINT,
        rest: REST_ENDPOINT,
      };

      await window.keplr.experimentalSuggestChain(chainInfo);
      await window.keplr.enable(CHAIN_ID);

      const offlineSigner = window.keplr.getOfflineSigner(CHAIN_ID);
      const newClient = await SigningCosmWasmClient.connectWithSigner(
        RPC_ENDPOINT,
        offlineSigner,
        { gasPrice: GAS_PRICE }
      );

      const accounts = await offlineSigner.getAccounts();
      if (accounts.length === 0) {
        throw new Error('No accounts found');
      }

      const newAddress = accounts[0].address;
      
      // Update all states together
      setClient(newClient);
      setAddress(newAddress);
      setIsConnected(true);
      localStorage.setItem('lastConnectedAddress', newAddress);
    } catch (err) {
      const contractError: ContractError = err instanceof Error ? err : new Error('Failed to connect');
      console.error('Connection error:', contractError);
      setError(contractError);
      setIsConnected(false);
      setClient(null);
      setAddress('');
      localStorage.removeItem('lastConnectedAddress');
      throw contractError;
    } finally {
      setIsLoading(false);
    }
  };

  const disconnect = useCallback(() => {
    setClient(null);
    setAddress('');
    setIsConnected(false);
    setError(null);
    localStorage.removeItem('lastConnectedAddress');
  }, []);

  const buyPixel = async (x: number, y: number): Promise<ExecuteResult> => {
    if (!client || !address) throw new Error('Not connected');
    const contractAddress = validateContractAddress(CONTRACT_ADDRESS);

    return await client.execute(
      address,
      contractAddress,
      {
        buy_pixel: { x, y }
      },
      'auto'
    );
  };

  const buyPixels = async (pixels: PixelData[]): Promise<ExecuteResult> => {
    if (!client || !address) throw new Error('Not connected');
    const contractAddress = validateContractAddress(CONTRACT_ADDRESS);

    return await client.execute(
      address,
      contractAddress,
      {
        buy_pixels: {
          pixels: pixels.map(p => ({
            x: p.x,
            y: p.y,
            color: p.color
          }))
        }
      },
      'auto'
    );
  };

  const estimateGas = async (pixels: PixelData[]): Promise<number> => {
    if (!client || !address) throw new Error('Not connected');
    const contractAddress = validateContractAddress(CONTRACT_ADDRESS);

    try {
      const msg = {
        buy_pixels: {
          pixels: pixels.map(p => ({
            x: p.x,
            y: p.y,
            color: p.color
          }))
        }
      };

      // Simulate the transaction to estimate gas
      const estimate = await client.simulate(
        address,
        [
          {
            typeUrl: "/cosmwasm.wasm.v1.MsgExecuteContract",
            value: {
              sender: address,
              contract: contractAddress,
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
      // Return a conservative estimate if simulation fails
      return Math.ceil((BASE_GAS + (pixels.length * GAS_PER_PIXEL)) * GAS_BUFFER);
    }
  };

  const getPixel = useCallback(async (x: number, y: number): Promise<PixelInfo> => {
    if (!client) {
      throw new Error('Client not initialized');
    }
    
    const contractAddr = validateContractAddress(CONTRACT_ADDRESS);
    const response = await client.queryContractSmart(contractAddr, {
      get_pixel: { x, y }
    });

    return response;
  }, [client]);

  const setPixelColor = async (x: number, y: number, color: string): Promise<ExecuteResult> => {
    if (!client || !address) throw new Error('Not connected');
    const contractAddress = validateContractAddress(CONTRACT_ADDRESS);

    return await client.execute(
      address,
      contractAddress,
      {
        set_pixel_color: { x, y, color }
      },
      'auto'
    );
  };

  const getCanvas = useCallback(async (): Promise<Array<[number, number, PixelInfo]>> => {
    if (!client) {
      throw new Error('Client not initialized');
    }

    const contractAddr = validateContractAddress(CONTRACT_ADDRESS);
    const response = await client.queryContractSmart(contractAddr, {
      get_canvas: {}
    });

    return response.pixels;
  }, [client]);

  return {
    isConnected,
    address,
    error,
    chainId: CHAIN_ID,
    isLoading,
    connect,
    disconnect,
    buyPixel,
    buyPixels,
    setPixelColor,
    getPixel,
    getCanvas,
    estimateGas,
  };
} 