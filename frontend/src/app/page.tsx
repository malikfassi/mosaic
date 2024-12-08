'use client';

import { useState } from 'react';
import dynamic from 'next/dynamic';

// Dynamically import components that require client-side functionality
const PixelCanvas = dynamic(() => import('@/components/PixelCanvas'), {
  ssr: false,
  loading: () => <div>Loading canvas...</div>
});

const WalletConnect = dynamic(() => import('@/components/WalletConnect'), {
  ssr: false,
  loading: () => <div>Loading wallet...</div>
});

export default function Home() {
  const [isConnected, setIsConnected] = useState(false);

  return (
    <div className="flex flex-col items-center justify-center p-8 md:p-24 space-y-8">
      <h1 className="text-4xl md:text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-purple-400 to-pink-600">
        Pixel Canvas
      </h1>
      
      <p className="text-lg text-gray-300 text-center max-w-2xl">
        Buy, own, and customize pixels on the Stargaze blockchain. Create your mark in this decentralized canvas!
      </p>
      
      <WalletConnect onConnected={() => setIsConnected(true)} />
      
      {isConnected && (
        <div className="w-full max-w-4xl mt-8">
          <PixelCanvas />
        </div>
      )}
    </div>
  );
} 