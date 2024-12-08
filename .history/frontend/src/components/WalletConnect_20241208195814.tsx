'use client';

import { useState } from 'react';
import { toast } from 'react-hot-toast';

interface WalletConnectProps {
  onConnected: () => void;
}

export default function WalletConnect({ onConnected }: WalletConnectProps) {
  const [connecting, setConnecting] = useState(false);

  const connectWallet = async () => {
    try {
      setConnecting(true);
      
      // Check if Keplr is installed
      if (!window.keplr) {
        toast.error('Please install Keplr extension');
        return;
      }

      // Enable Stargaze chain
      await window.keplr.enable('stargaze-1');
      
      // Get the offline signer
      const offlineSigner = window.keplr.getOfflineSigner('stargaze-1');
      
      // Get the user's address
      const accounts = await offlineSigner.getAccounts();
      
      if (accounts.length > 0) {
        toast.success('Wallet connected!');
        onConnected();
      }
    } catch (error) {
      console.error('Error connecting wallet:', error);
      toast.error('Failed to connect wallet');
    } finally {
      setConnecting(false);
    }
  };

  return (
    <button
      onClick={connectWallet}
      disabled={connecting}
      className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
    >
      {connecting ? 'Connecting...' : 'Connect Wallet'}
    </button>
  );
} 