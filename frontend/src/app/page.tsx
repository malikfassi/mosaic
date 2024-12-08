'use client';

import PixelCanvas from '@/components/PixelCanvas';
import WalletConnect from '@/components/WalletConnect';
import { toast } from 'react-hot-toast';

export default function Home() {
  const handleWalletConnect = (address: string) => {
    toast.success(`Connected: ${address.slice(0, 8)}...${address.slice(-8)}`);
  };

  return (
    <main className="min-h-screen p-8">
      <div className="container mx-auto">
        <h1 className="text-4xl font-bold mb-8">Pixel Canvas</h1>
        <div className="grid grid-cols-1 gap-8">
          <PixelCanvas width={100} height={100} pixelSize={5} />
          <WalletConnect onConnected={handleWalletConnect} />
        </div>
      </div>
    </main>
  );
} 