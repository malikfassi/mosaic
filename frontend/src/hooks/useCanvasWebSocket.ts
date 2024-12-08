import { useEffect, useRef, useState, useCallback } from 'react';
import { toast } from 'react-hot-toast';

interface WebSocketMessage {
  type: string;
  data: {
    x: number;
    y: number;
    color: string;
  };
}

export function useCanvasWebSocket(url: string) {
  const [isConnected, setIsConnected] = useState(false);
  const wsRef = useRef<WebSocket | null>(null);

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN) return;

    const ws = new WebSocket(url);
    wsRef.current = ws;

    ws.onopen = () => {
      setIsConnected(true);
      toast.success('WebSocket connected!');
    };

    ws.onclose = () => {
      setIsConnected(false);
      toast.error('WebSocket disconnected');
      setTimeout(connect, 5000);
    };

    ws.onerror = (error) => {
      toast.error('WebSocket error occurred');
      // Log error for debugging
      if (process.env.NODE_ENV === 'development') {
        // eslint-disable-next-line no-console
        console.error('WebSocket error:', error);
      }
    };

    ws.onmessage = (event) => {
      try {
        const message: WebSocketMessage = JSON.parse(event.data);
        // Handle message
        if (process.env.NODE_ENV === 'development') {
          // eslint-disable-next-line no-console
          console.log('Received message:', message);
        }
      } catch (error) {
        toast.error('Error parsing WebSocket message');
        if (process.env.NODE_ENV === 'development') {
          // eslint-disable-next-line no-console
          console.error('Error parsing message:', error);
        }
      }
    };
  }, [url]);

  useEffect(() => {
    connect();

    return () => {
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, [connect]);

  const sendMessage = (message: WebSocketMessage) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    } else {
      toast.error('WebSocket is not connected');
    }
  };

  return {
    isConnected,
    sendMessage
  };
} 