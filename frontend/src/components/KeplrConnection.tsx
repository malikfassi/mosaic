'use client';

import { useContract } from '@/hooks/useContract';
import { Button } from '@/components/ui/button';
import { useToast } from '@/components/ui/use-toast';

export function KeplrConnection() {
  const { isConnected, address, connect, disconnect, error } = useContract();
  const { toast } = useToast();

  const handleConnect = async () => {
    try {
      await connect();
      toast({
        title: 'Connected to Keplr',
        description: 'Successfully connected to your Keplr wallet.',
      });
    } catch (err) {
      toast({
        title: 'Connection Failed',
        description: err instanceof Error ? err.message : 'Failed to connect to Keplr',
        variant: 'destructive',
      });
    }
  };

  const handleDisconnect = () => {
    disconnect();
    toast({
      title: 'Disconnected',
      description: 'Successfully disconnected from Keplr wallet.',
    });
  };

  if (error) {
    return (
      <Button variant="destructive" onClick={handleConnect}>
        Retry Connection
      </Button>
    );
  }

  if (isConnected && address) {
    return (
      <div className="flex items-center gap-2">
        <span className="text-sm text-muted-foreground">
          {address === 'stars1mock...' ? address : `${address.slice(0, 6)}...${address.slice(-4)}`}
        </span>
        <Button variant="outline" onClick={handleDisconnect}>
          Disconnect
        </Button>
      </div>
    );
  }

  return (
    <Button onClick={handleConnect}>
      Connect Keplr
    </Button>
  );
} 