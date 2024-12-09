'use client';

import { FC, useEffect, useRef, useState } from 'react';
<<<<<<< HEAD
import { useCanvasWebSocket } from '@/hooks/useCanvasWebSocket';
=======
import { toast } from 'react-hot-toast';
import { useContract } from '@/hooks/useContract';
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)

interface PixelCanvasProps {
  width: number;
  height: number;
  pixelSize: number;
}

<<<<<<< HEAD
const PixelCanvas: FC<PixelCanvasProps> = ({ width, height, pixelSize }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [selectedColor, setSelectedColor] = useState('#000000');
  const { isConnected } = useCanvasWebSocket(process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001');
=======
interface PendingPixel {
  x: number;
  y: number;
  color: string;
  timestamp: number;
}

const PIXEL_PRICE = 10; // 10 STARS per pixel

const PixelCanvas: FC<PixelCanvasProps> = ({ width, height, pixelSize }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [selectedColor, setSelectedColor] = useState('#000000');
  const [pendingPixels, setPendingPixels] = useState<PendingPixel[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const { isConnected, buyPixels, estimateGas } = useContract();
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = width * pixelSize;
    canvas.height = height * pixelSize;

<<<<<<< HEAD
=======
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
    // Draw grid
    ctx.strokeStyle = '#ddd';
    for (let x = 0; x <= width; x++) {
      ctx.beginPath();
      ctx.moveTo(x * pixelSize, 0);
      ctx.lineTo(x * pixelSize, height * pixelSize);
      ctx.stroke();
    }
    for (let y = 0; y <= height; y++) {
      ctx.beginPath();
      ctx.moveTo(0, y * pixelSize);
      ctx.lineTo(width * pixelSize, y * pixelSize);
      ctx.stroke();
    }
<<<<<<< HEAD
  }, [width, height, pixelSize]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
=======

    // Draw pending pixels with fade-in animation
    pendingPixels.forEach(pixel => {
      const age = Date.now() - pixel.timestamp;
      const alpha = Math.min(1, age / 300); // 300ms fade-in
      ctx.fillStyle = pixel.color + Math.floor(alpha * 255).toString(16).padStart(2, '0');
      ctx.fillRect(pixel.x * pixelSize, pixel.y * pixelSize, pixelSize, pixelSize);
    });
  }, [width, height, pixelSize, pendingPixels]);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isConnected) {
      toast.error('Please connect your wallet first');
      return;
    }

    if (isSubmitting) {
      toast.error('Please wait for the current transaction to complete');
      return;
    }

>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / pixelSize);
    const y = Math.floor((event.clientY - rect.top) / pixelSize);

<<<<<<< HEAD
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Fill pixel
    ctx.fillStyle = selectedColor;
    ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);
  };

  return (
    <div className="relative">
      <canvas
        ref={canvasRef}
        onClick={handleCanvasClick}
        className="border border-gray-300"
      />
      <div className="absolute top-0 right-0 p-4 bg-white rounded-lg shadow-md">
=======
    // Check if pixel is already pending
    if (pendingPixels.some(p => p.x === x && p.y === y)) {
      toast.error('Pixel already selected');
      return;
    }

    // Add to pending pixels with timestamp
    setPendingPixels(prev => [...prev, { x, y, color: selectedColor, timestamp: Date.now() }]);
  };

  const handleUndo = () => {
    setPendingPixels(prev => prev.slice(0, -1));
  };

  const handleClearPending = () => {
    setPendingPixels([]);
  };

  const getTotalCost = () => {
    return pendingPixels.length * PIXEL_PRICE;
  };

  const handleSubmitTransaction = async () => {
    if (!isConnected) {
      toast.error('Please connect your wallet first');
      return;
    }

    if (pendingPixels.length === 0) {
      toast.error('No pixels selected');
      return;
    }

    try {
      setIsSubmitting(true);
      const toastId = toast.loading('Preparing transaction...');

      // Estimate gas first
      const gasEstimate = await estimateGas(pendingPixels);
      toast.loading(`Estimated gas: ${gasEstimate}`, { id: toastId });
      
      // Process all pixels in a single transaction
      const pixels = pendingPixels.map(p => ({
        x: p.x,
        y: p.y,
        color: p.color
      }));
      
      await buyPixels(pixels);

      toast.success('Transaction successful!', { id: toastId });
      setPendingPixels([]); // Clear pending pixels after successful transaction
    } catch (error) {
      console.error('Transaction failed:', error);
      toast.error('Transaction failed: ' + (error instanceof Error ? error.message : 'Unknown error'));
    } finally {
      setIsSubmitting(false);
    }
  };

  return (
    <div className="flex flex-col items-center gap-4">
      <div className="relative">
        <canvas
          ref={canvasRef}
          onClick={handleCanvasClick}
          className={`border border-gray-300 cursor-pointer ${isSubmitting ? 'opacity-50' : ''}`}
        />
        {isSubmitting && (
          <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-30">
            <div className="animate-spin rounded-full h-12 w-12 border-4 border-white border-t-transparent"></div>
          </div>
        )}
      </div>
      
      <div className="flex items-center gap-4">
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
        <input
          type="color"
          value={selectedColor}
          onChange={(e) => setSelectedColor(e.target.value)}
<<<<<<< HEAD
          className="w-8 h-8 cursor-pointer"
        />
        <div className="mt-2 text-sm text-gray-600">
          {isConnected ? 'Connected' : 'Disconnected'}
        </div>
=======
          className="w-20 h-10"
          disabled={isSubmitting}
        />
        
        {pendingPixels.length > 0 && (
          <div className="flex flex-col gap-2">
            <p className="text-sm">
              Selected: {pendingPixels.length} pixels
              <br />
              Total Cost: {getTotalCost()} STARS
            </p>
            <div className="flex gap-2">
              <button
                onClick={handleUndo}
                disabled={isSubmitting}
                className="px-3 py-1 bg-gray-500 text-white rounded-lg hover:bg-gray-600
                  transform transition-all duration-200 hover:scale-105 active:scale-95
                  disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Undo
              </button>
              <button
                onClick={handleClearPending}
                disabled={isSubmitting}
                className="px-3 py-1 bg-red-500 text-white rounded-lg hover:bg-red-600
                  transform transition-all duration-200 hover:scale-105 active:scale-95
                  disabled:opacity-50 disabled:cursor-not-allowed"
              >
                Clear
              </button>
              <button
                onClick={handleSubmitTransaction}
                disabled={isSubmitting}
                className="px-4 py-2 bg-purple-500 text-white rounded-lg hover:bg-purple-600
                  transform transition-all duration-200 hover:scale-105 active:scale-95
                  disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {isSubmitting ? 'Processing...' : 'Submit Transaction'}
              </button>
            </div>
          </div>
        )}
>>>>>>> 5a17691 (feat: implement contract interactions and remove websocket - Add contract methods, remove WS for MVP, update env config, add batch transactions)
      </div>
    </div>
  );
};

export default PixelCanvas; 