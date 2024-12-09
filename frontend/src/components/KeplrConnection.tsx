import { FC, useState, useEffect } from 'react';
import { toast } from 'react-hot-toast';
import { useContract } from '@/hooks/useContract';

interface KeplrConnectionProps {
  onConnected: (address: string) => void;
  connecting?: boolean;
}

const KeplrConnection: FC<KeplrConnectionProps> = ({ onConnected, connecting = false }) => {
  const [error, setError] = useState<string>('');
  const [balance, setBalance] = useState<string>('');
  const { connect: connectContract, isConnected, address, disconnect, isLoading } = useContract();

  useEffect(() => {
    // Call onConnected when address is available and connected
    if (address && isConnected) {
      onConnected(address);
    }
  }, [address, isConnected, onConnected]);

  const fetchBalance = async (address: string) => {
    try {
      const restEndpoint = process.env.NEXT_PUBLIC_STARGAZE_REST;
      if (!restEndpoint) throw new Error('REST endpoint not configured');

      // Remove trailing slash if present
      const baseUrl = restEndpoint.replace(/\/$/, '');
      const url = `${baseUrl}/cosmos/bank/v1beta1/balances/${address}`;
      
      console.log('Fetching balance from:', url);
      
      const response = await fetch(url);
      
      if (!response.ok) {
        const errorText = await response.text();
        console.error('Balance fetch failed:', {
          status: response.status,
          statusText: response.statusText,
          error: errorText
        });
        throw new Error(`HTTP error! status: ${response.status} - ${errorText}`);
      }
      
      const data = await response.json();
      console.log('Balance response:', data);
      
      // Find STARS balance - handle both array and object response formats
      const balances = Array.isArray(data.balances) ? data.balances : [data];
      const ustarsBalance = balances.find(
        (b: any) => b.denom === 'ustars'
      )?.amount || '0';
      
      // Convert uSTARS to STARS (1 STARS = 1,000,000 uSTARS)
      const starsBalance = (parseInt(ustarsBalance) / 1_000_000).toFixed(2);
      setBalance(starsBalance);
    } catch (error) {
      console.error('Error fetching balance:', error);
      setBalance('0.00');
      // Show error toast only in development
      if (process.env.NODE_ENV === 'development') {
        toast.error(`Balance fetch failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
      }
    }
  };

  useEffect(() => {
    if (address && isConnected) {
      fetchBalance(address);
      const interval = setInterval(() => fetchBalance(address), 10000); // Refresh every 10s
      return () => clearInterval(interval);
    }
  }, [address, isConnected]);

  const connectWallet = async () => {
    try {
      if (!window.keplr) {
        throw new Error('Keplr wallet not found');
      }
      
      const toastId = toast.loading('Connecting wallet...');
      await connectContract();
      toast.success('Wallet connected!', {
        id: toastId,
        icon: '🌟',
        duration: 3000,
      });
    } catch (error) {
      setError('Failed to connect');
      console.error('Error connecting wallet:', error);
      toast.error(error instanceof Error ? error.message : 'Failed to connect wallet', {
        duration: 5000,
        icon: '❌',
      });
    }
  };

  return (
    <div className="flex flex-col items-center gap-2">
      <button
        onClick={isConnected ? disconnect : connectWallet}
        disabled={connecting || isLoading}
        className="relative px-6 py-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white font-bold rounded-lg 
          hover:from-purple-600 hover:to-pink-600 disabled:opacity-50 disabled:cursor-not-allowed
          transform transition-all duration-200 hover:scale-105 active:scale-95"
      >
        {connecting || isLoading ? (
          <>
            <span className="opacity-0">Connect Wallet</span>
            <div className="absolute inset-0 flex items-center justify-center">
              <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-white"></div>
            </div>
          </>
        ) : address && isConnected ? (
          `${address.slice(0, 8)}...${address.slice(-8)}`
        ) : (
          'Connect Wallet'
        )}
      </button>
      
      {balance && isConnected && (
        <div className="text-sm font-medium">
          Balance: {balance} STARS
        </div>
      )}
      
      {error && (
        <p className="error text-red-500 text-sm mt-2">{error}</p>
      )}
    </div>
  );
};

export default KeplrConnection; 