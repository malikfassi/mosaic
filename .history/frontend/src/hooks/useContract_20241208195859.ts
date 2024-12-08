import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { toast } from 'react-hot-toast';

const CONTRACT_ADDRESS = process.env.NEXT_PUBLIC_CONTRACT_ADDRESS;

export function useContract() {
  const getClient = async () => {
    if (!window.keplr) {
      throw new Error('Keplr wallet not found');
    }

    await window.keplr.enable('stargaze-1');
    const offlineSigner = window.keplr.getOfflineSigner('stargaze-1');
    const client = await SigningCosmWasmClient.connectWithSigner(
      'https://rpc.stargaze-apis.com/',
      offlineSigner
    );

    return client;
  };

  const buyPixel = async (x: number, y: number) => {
    try {
      const client = await getClient();
      const accounts = await window.keplr!.getOfflineSigner('stargaze-1').getAccounts();
      const sender = accounts[0].address;

      const result = await client.execute(
        sender,
        CONTRACT_ADDRESS!,
        { buy_pixel: { x, y } },
        'auto'
      );

      return result;
    } catch (error) {
      console.error('Error buying pixel:', error);
      throw error;
    }
  };

  const setPixelColor = async (x: number, y: number, color: string) => {
    try {
      const client = await getClient();
      const accounts = await window.keplr!.getOfflineSigner('stargaze-1').getAccounts();
      const sender = accounts[0].address;

      const result = await client.execute(
        sender,
        CONTRACT_ADDRESS!,
        { set_pixel_color: { x, y, color } },
        'auto'
      );

      return result;
    } catch (error) {
      console.error('Error setting pixel color:', error);
      throw error;
    }
  };

  const getPixel = async (x: number, y: number) => {
    try {
      const client = await getClient();
      const result = await client.queryContractSmart(CONTRACT_ADDRESS!, {
        get_pixel: { x, y },
      });
      return result;
    } catch (error) {
      console.error('Error getting pixel:', error);
      throw error;
    }
  };

  const getCanvas = async () => {
    try {
      const client = await getClient();
      const result = await client.queryContractSmart(CONTRACT_ADDRESS!, {
        get_canvas: {},
      });
      return result;
    } catch (error) {
      console.error('Error getting canvas:', error);
      throw error;
    }
  };

  return {
    buyPixel,
    setPixelColor,
    getPixel,
    getCanvas,
  };
} 