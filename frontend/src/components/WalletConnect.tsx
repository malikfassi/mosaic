'use client';

import { useState } from 'react';
import { toast } from 'react-hot-toast';
import { STARGAZE_CHAIN_ID, getStargazeChainInfo } from '@/config/chain';
import type { WalletInfo } from '@/types';

interface WalletConnectProps {
  onConnected: (address: string) => void;
  connecting?: boolean;
}

export default function WalletConnect({ onConnected, connecting = false }: WalletConnectProps) {
  const connectWallet = async () => {
    try {
      // Check if Keplr is installed
      if (!window.keplr) {
        toast.error('Please install Keplr extension', {
          duration: 5000,
          icon: '🦊',
        });
        return;
      }

      // Suggest chain to Keplr
      await window.keplr.experimentalSuggestChain(getStargazeChainInfo());

      // Enable Stargaze chain
      await window.keplr.enable(STARGAZE_CHAIN_ID);
      
      // Get the offline signer
      const offlineSigner = window.keplr.getOfflineSigner(STARGAZE_CHAIN_ID);
      
      // Get the user's address
      const accounts = await offlineSigner.getAccounts();
      
      if (accounts.length > 0) {
        const address = accounts[0].address;
        toast.success('Wallet connected!', {
          icon: '🌟',
          duration: 3000,
        });
        
        onConnected(address);
      } else {
        throw new Error('No accounts found');
      }
    } catch (error) {
      console.error('Error connecting wallet:', error);
      toast.error(
        error instanceof Error ? error.message : 'Failed to connect wallet',
        {
          duration: 5000,
          icon: '❌',
        }
      );
    }
  };

  return (
    <button
      onClick={connectWallet}
      disabled={connecting}
      className="relative px-6 py-3 bg-gradient-to-r from-purple-500 to-pink-500 text-white font-bold rounded-lg 
        hover:from-purple-600 hover:to-pink-600 disabled:opacity-50 disabled:cursor-not-allowed
        transform transition-all duration-200 hover:scale-105 active:scale-95"
    >
      {connecting ? (
        <>
          <span className="opacity-0">Connect Wallet</span>
          <div className="absolute inset-0 flex items-center justify-center">
            <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-white"></div>
          </div>
        </>
      ) : (
        'Connect Wallet'
      )}
    </button>
  );
} 