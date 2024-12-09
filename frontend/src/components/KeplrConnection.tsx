'use client';

import { useContract } from '@/hooks/useContract';
import { toast } from 'react-hot-toast';

export default function KeplrConnection() {
  const { 
    isConnected, 
    isInitialized,
    connect, 
    disconnect, 
    address,
    error,
    transactionStatus 
  } = useContract();

  const connectWallet = async () => {
    const toastId = toast.loading('Connecting to Keplr...');
    try {
      await connect();
      toast.success('Connected to Keplr!', { id: toastId });
    } catch (err: any) {
      let errorMessage = 'Failed to connect to Keplr';
      
      // Handle specific error cases
      if (err.message?.includes('not found')) {
        errorMessage = 'Keplr wallet not found. Please install Keplr extension.';
      } else if (err.message?.includes('suggest chain')) {
        errorMessage = 'Failed to configure chain in Keplr. Please try again.';
      } else if (err.message?.includes('enable')) {
        errorMessage = 'Please approve the connection in Keplr.';
      } else if (err.message?.includes('No accounts')) {
        errorMessage = 'No accounts found. Please add an account to Keplr.';
      } else if (err.details) {
        errorMessage = `${err.message}: ${err.details}`;
      }

      console.error('Error connecting wallet:', {
        message: err.message,
        type: err.type,
        details: err.details,
        cause: err.cause
      });
      
      toast.error(errorMessage, { id: toastId });
    }
  };

  const disconnectWallet = () => {
    disconnect();
    toast.success('Disconnected from Keplr');
  };

  // Show connection status
  const getConnectionStatus = () => {
    if (transactionStatus.connect.isLoading) {
      return 'Connecting...';
    }
    if (isConnected && !isInitialized) {
      return 'Initializing...';
    }
    if (isConnected && isInitialized) {
      return 'Connected';
    }
    return 'Connect Wallet';
  };

  // Format address for display
  const formatAddress = (addr: string) => {
    if (!addr) return '';
    return `${addr.slice(0, 8)}...${addr.slice(-4)}`;
  };

  return (
    <div className="flex items-center gap-4">
      {isConnected ? (
        <>
          <span className="text-sm text-gray-600">
            {formatAddress(address)}
          </span>
          <button
            onClick={disconnectWallet}
            className="px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 
              transition-colors duration-200 disabled:opacity-50"
            disabled={transactionStatus.connect.isLoading}
          >
            Disconnect
          </button>
        </>
      ) : (
        <button
          onClick={connectWallet}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 
            transition-colors duration-200 disabled:opacity-50"
          disabled={transactionStatus.connect.isLoading}
        >
          {getConnectionStatus()}
        </button>
      )}
      {error && (
        <span className="text-sm text-red-500">
          {error.message}
        </span>
      )}
    </div>
  );
} 