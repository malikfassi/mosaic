'use client';

import { FC } from 'react';
import KeplrConnection from './KeplrConnection';

interface WalletConnectProps {
  onConnected: (address: string) => void;
  connecting?: boolean;
}

const WalletConnect: FC<WalletConnectProps> = ({ onConnected, connecting = false }) => {
  return (
    <div>
      <KeplrConnection onConnected={onConnected} connecting={connecting} />
    </div>
  );
};

export default WalletConnect; 