'use client';

import { useState } from 'react';
import dynamic from 'next/dynamic';
import type { WalletInfo } from '@/types';

// Dynamically import components that require client-side functionality
const PixelCanvas = dynamic(() => import('@/components/PixelCanvas'), {
  ssr: false,
  loading: () => (
    <div className="w-full h-[500px] bg-gray-900/50 rounded-lg animate-pulse flex items-center justify-center">
      <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-white"></div>
    </div>
  ),
});

const WalletConnect = dynamic(() => import('@/components/WalletConnect'), {
  ssr: false,
  loading: () => (
    <div className="h-12 w-40 bg-gray-900/50 rounded-lg animate-pulse"></div>
  ),
});

export default function Home() {
  const [walletInfo, setWalletInfo] = useState<WalletInfo | null>(null);

  return (
    <div className="flex flex-col items-center justify-center p-8 md:p-24 space-y-8">
      <h1 className="text-4xl md:text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-600">
        Pixel Canvas
      </h1>
      
      <p className="text-lg text-gray-300 text-center max-w-2xl">
        Buy, own, and customize pixels on the Stargaze blockchain. Create your mark in this decentralized canvas!
      </p>
      
      {walletInfo ? (
        <div className="flex flex-col items-center space-y-2">
          <div className="text-sm text-gray-400">
            Connected: {walletInfo.address.slice(0, 8)}...{walletInfo.address.slice(-8)}
          </div>
          <div className="w-full max-w-4xl">
            <PixelCanvas />
          </div>
        </div>
      ) : (
        <div className="flex flex-col items-center space-y-4">
          <WalletConnect onConnected={setWalletInfo} />
          <p className="text-sm text-gray-500">
            Connect your wallet to start customizing the canvas
          </p>
        </div>
      )}
    </div>
  );
} 