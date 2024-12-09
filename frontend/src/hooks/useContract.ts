import { SigningCosmWasmClient, ExecuteResult } from '@cosmjs/cosmwasm-stargate';
import { useState, useCallback } from 'react';
import { STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';

<<<<<<< HEAD
=======
interface PixelData {
  x: number;
  y: number;
  color: string;
}

>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
interface ContractHookResult {
  isConnected: boolean;
  address: string;
  error: Error | null;
  chainId: string;
  connect: () => Promise<void>;
  disconnect: () => void;
  buyPixel: (x: number, y: number) => Promise<ExecuteResult>;
<<<<<<< HEAD
  setPixelColor: (x: number, y: number, color: string) => Promise<ExecuteResult>;
  getPixel: (x: number, y: number) => Promise<unknown>;
  getCanvas: () => Promise<unknown>;
}

=======
  buyPixels: (pixels: PixelData[]) => Promise<ExecuteResult>;
  setPixelColor: (x: number, y: number, color: string) => Promise<ExecuteResult>;
  getPixel: (x: number, y: number) => Promise<unknown>;
  getCanvas: () => Promise<unknown>;
  estimateGas: (pixels: PixelData[]) => Promise<number>;
}

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS;
const CHAIN_ID = process.env.NEXT_PUBLIC_STARGAZE_CHAIN_ID || 'elgafar-1';
const RPC_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_RPC || 'https://rpc.elgafar-1.stargaze-apis.com';
const REST_ENDPOINT = process.env.NEXT_PUBLIC_STARGAZE_REST || 'https://rest.elgafar-1.stargaze-apis.com';

>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
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

<<<<<<< HEAD
      // Suggest chain to Keplr
      await window.keplr.experimentalSuggestChain(getStargazeChainInfo());
      await window.keplr.enable(STARGAZE_CHAIN_ID);

      const offlineSigner = window.keplr.getOfflineSigner(STARGAZE_CHAIN_ID);
      const client = await SigningCosmWasmClient.connectWithSigner(
        getStargazeChainInfo().rpc,
=======
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
      const client = await SigningCosmWasmClient.connectWithSigner(
        RPC_ENDPOINT,
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
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

<<<<<<< HEAD
  const getPixel = async (_x: number, _y: number) => {
    if (!client) throw new Error('Not connected');
    // Implementation
    return {};
=======
  const buyPixel = async (x: number, y: number) => {
    if (!client || !address) throw new Error('Not connected');
    if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

    return await client.execute(
      address,
      CONTRACT_ADDRESS,
      {
        buy_pixel: {
          x: x,
          y: y
        }
      },
      'auto'
    );
  };

  const buyPixels = async (pixels: PixelData[]) => {
    if (!client || !address) throw new Error('Not connected');
    if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

    return await client.execute(
      address,
      CONTRACT_ADDRESS,
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

  const estimateGas = async (pixels: PixelData[]) => {
    if (!client || !address) throw new Error('Not connected');
    if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

    try {
      const gasEstimate = await client.simulate(
        address,
        CONTRACT_ADDRESS,
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
      return gasEstimate;
    } catch (error) {
      console.error('Gas estimation failed:', error);
      // Return a conservative estimate if simulation fails
      const baseCost = 100000;
      const perPixelCost = 50000;
      return baseCost + (pixels.length * perPixelCost);
    }
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
  };

  return {
    isConnected,
    address,
    error,
<<<<<<< HEAD
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
=======
    chainId: CHAIN_ID,
    connect,
    disconnect,
    buyPixel,
    buyPixels,
    setPixelColor: async (x: number, y: number, color: string) => {
      if (!client || !address) throw new Error('Not connected');
      if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

      return await client.execute(
        address,
        CONTRACT_ADDRESS,
        {
          set_pixel_color: {
            x,
            y,
            color
          }
        },
        'auto'
      );
    },
    getPixel: async (x: number, y: number) => {
      if (!client) throw new Error('Not connected');
      if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

      return await client.queryContractSmart(CONTRACT_ADDRESS, {
        get_pixel: { x, y }
      });
    },
    getCanvas: async () => {
      if (!client) throw new Error('Not connected');
      if (!CONTRACT_ADDRESS) throw new Error('Contract address not configured');

      return await client.queryContractSmart(CONTRACT_ADDRESS, {
        get_canvas: {}
      });
    },
    estimateGas
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
  };
} 