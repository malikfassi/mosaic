import { FC, useState } from 'react';
import { toast } from 'react-hot-toast';

interface KeplrConnectionProps {
  onConnected: (address: string) => void;
  connecting?: boolean;
}

const KeplrConnection: FC<KeplrConnectionProps> = ({ onConnected, connecting = false }) => {
  const [error, setError] = useState<string>('');

  const connectWallet = async () => {
    try {
      if (!window.keplr) {
        throw new Error('Keplr wallet not found');
      }
      
      await window.keplr.enable('cosmoshub-4');
      const key = await window.keplr.getKey('cosmoshub-4');
      onConnected(key.bech32Address);
      toast.success('Wallet connected!', {
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
      {error && <p className="error absolute top-full left-0 right-0 mt-2 text-red-500 text-sm">{error}</p>}
    </button>
  );
};

export default KeplrConnection; 