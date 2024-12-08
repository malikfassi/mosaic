import { useEffect, useRef, useState } from 'react';
import { toast } from 'react-hot-toast';
import type { Pixel, PixelCoordinates } from '@/types';

interface CanvasUpdate {
  type: 'pixel_update' | 'pixel_purchase';
  x: number;
  y: number;
  pixel: Pixel;
}

export function useCanvasWebSocket(onUpdate: (update: CanvasUpdate) => void) {
  const wsRef = useRef<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);
  const reconnectTimeoutRef = useRef<NodeJS.Timeout>();

  const connect = () => {
    // Use secure WebSocket for production
    const protocol = process.env.NODE_ENV === 'production' ? 'wss' : 'ws';
    const wsUrl = `${protocol}://${process.env.NEXT_PUBLIC_WS_URL || 'localhost:3001'}/canvas`;

    const ws = new WebSocket(wsUrl);

    ws.onopen = () => {
      setIsConnected(true);
      toast.success('Real-time updates connected!', {
        id: 'ws-connection',
        duration: 2000,
        icon: '🔄',
      });
    };

    ws.onclose = () => {
      setIsConnected(false);
      // Try to reconnect after 5 seconds
      reconnectTimeoutRef.current = setTimeout(connect, 5000);
      toast.error('Real-time updates disconnected. Reconnecting...', {
        id: 'ws-connection',
        duration: 3000,
        icon: '🔌',
      });
    };

    ws.onerror = (error) => {
      console.error('WebSocket error:', error);
      ws.close();
    };

    ws.onmessage = (event) => {
      try {
        const update: CanvasUpdate = JSON.parse(event.data);
        onUpdate(update);

        // Show toast for pixel updates
        if (update.type === 'pixel_purchase') {
          toast.success(
            `Pixel (${update.x}, ${update.y}) purchased by ${update.pixel.owner.slice(0, 8)}...`,
            {
              duration: 3000,
              icon: '🎨',
            }
          );
        }
      } catch (error) {
        console.error('Error processing WebSocket message:', error);
      }
    };

    wsRef.current = ws;
  };

  useEffect(() => {
    connect();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
    };
  }, []);

  // Function to send updates to the WebSocket server
  const sendUpdate = (x: number, y: number, pixel: Pixel) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(
        JSON.stringify({
          type: 'pixel_update',
          x,
          y,
          pixel,
        })
      );
    }
  };

  return {
    isConnected,
    sendUpdate,
  };
} 