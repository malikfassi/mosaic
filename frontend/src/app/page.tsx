'use client';

import { useEffect, useState } from 'react';
import { Toaster } from 'react-hot-toast';
import PixelCanvas from '@/components/PixelCanvas';
import WalletConnect from '@/components/WalletConnect';

export default function Home() {
  const [isConnected, setIsConnected] = useState(false);

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <Toaster position="top-right" />
      
      <h1 className="text-4xl font-bold mb-8">Pixel Canvas</h1>
      
      <WalletConnect onConnected={() => setIsConnected(true)} />
      
      {isConnected && (
        <div className="mt-8">
          <PixelCanvas />
        </div>
      )}
    </main>
  );
} 