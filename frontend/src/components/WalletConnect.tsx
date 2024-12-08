'use client';

import { FC, Component } from 'react';
import KeplrConnection from './KeplrConnection';

// Add LoadingSpinner component
const LoadingSpinner: FC = () => (
  <div className="flex justify-center items-center">
    <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
  </div>
);

interface WalletConnectProps {
  onConnected: (address: string) => void;
  connecting?: boolean;
}

interface ErrorBoundaryState {
  hasError: boolean;
  error?: Error;
}

class WalletErrorBoundary extends Component<{ children: React.ReactNode }, ErrorBoundaryState> {
  constructor(props: { children: React.ReactNode }) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): ErrorBoundaryState {
    return {
      hasError: true,
      error,
    };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo): void {
    console.error('Wallet error:', error, errorInfo);
  }

  render(): React.ReactNode {
    if (this.state.hasError) {
      return (
        <div className="text-red-500 p-4">
          <h2>Something went wrong with the wallet connection.</h2>
          <button
            className="mt-2 px-4 py-2 bg-blue-500 text-white rounded"
            onClick={() => this.setState({ hasError: false })}
          >
            Try again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

const WalletConnect: FC<WalletConnectProps> = ({ onConnected, connecting = false }) => {
  return (
    <WalletErrorBoundary>
      {connecting && <LoadingSpinner />}
      <div>
        <KeplrConnection onConnected={onConnected} connecting={connecting} />
      </div>
    </WalletErrorBoundary>
  );
};

export default WalletConnect; 