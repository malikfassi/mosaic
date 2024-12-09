'use client';

import { useRef, useEffect, useState, useCallback } from 'react';
import { toast } from 'react-hot-toast';
import { useContract } from '@/hooks/useContract';
import { debounce } from 'lodash';

interface PixelCanvasProps {
  width: number;
  height: number;
  pixelSize: number;
}

interface PendingPixel {
  x: number;
  y: number;
  color: string;
}

export default function PixelCanvas({ width, height, pixelSize }: PixelCanvasProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isDrawing, setIsDrawing] = useState(false);
  const [currentColor, setCurrentColor] = useState('#000000');
  const [pendingPixels, setPendingPixels] = useState<PendingPixel[]>([]);
  const [isSubmitting, setIsSubmitting] = useState(false);
  const [isLoadingPixels, setIsLoadingPixels] = useState(false);
  const { 
    isConnected, 
    isInitialized,
    setPixelColor,
    getPixelColor,
    getCanvas,
    estimateGas,
    transactionStatus 
  } = useContract();

  // Initialize canvas
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = width * pixelSize;
    canvas.height = height * pixelSize;

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
  }, [width, height, pixelSize]);

  // Load existing pixels when client is initialized
  useEffect(() => {
    if (isInitialized) {
      loadExistingPixels();
    }
  }, [isInitialized]);

  // Load and draw existing pixels from the contract
  const loadExistingPixels = async () => {
    if (isLoadingPixels) return;
    setIsLoadingPixels(true);

    try {
      const pixels = await getCanvas();
      const canvas = canvasRef.current;
      if (!canvas) return;

      const ctx = canvas.getContext('2d');
      if (!ctx) return;

      // Clear existing pixels before redrawing
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Redraw grid
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

      // Draw pixels
      pixels.forEach((pixel) => {
        ctx.fillStyle = pixel.color;
        ctx.fillRect(pixel.x * pixelSize, pixel.y * pixelSize, pixelSize, pixelSize);
      });
    } catch (error) {
      console.error('Failed to load pixels:', error);
      toast.error('Failed to load existing pixels');
    } finally {
      setIsLoadingPixels(false);
    }
  };

  // Handle mouse events for drawing
  const startDrawing = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isConnected) {
      toast.error('Please connect your wallet first');
      return;
    }
    setIsDrawing(true);
    const { x, y } = getPixelCoordinates(e);
    addPendingPixel(x, y);
  };

  const draw = (e: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDrawing || !isConnected) return;
    const { x, y } = getPixelCoordinates(e);
    addPendingPixel(x, y);
  };

  const stopDrawing = () => {
    setIsDrawing(false);
  };

  // Get pixel coordinates from mouse event
  const getPixelCoordinates = (e: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return { x: 0, y: 0 };

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((e.clientX - rect.left) / pixelSize);
    const y = Math.floor((e.clientY - rect.top) / pixelSize);
    return { x, y };
  };

  // Add pixel to pending list and draw preview
  const addPendingPixel = (x: number, y: number) => {
    if (x < 0 || x >= width || y < 0 || y >= height) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Draw preview
    ctx.fillStyle = currentColor;
    ctx.fillRect(x * pixelSize, y * pixelSize, pixelSize, pixelSize);

    // Add to pending list if not already present
    setPendingPixels(prev => {
      const exists = prev.some(p => p.x === x && p.y === y);
      if (exists) return prev;
      return [...prev, { x, y, color: currentColor }];
    });
  };

  // Submit pending pixels to the contract
  const submitPixels = async () => {
    if (!isConnected || pendingPixels.length === 0 || isSubmitting) return;

    const toastId = toast.loading('Preparing transaction...');
    setIsSubmitting(true);

    try {
      // Process pixels one by one
      for (const pixel of pendingPixels) {
        const gasEstimate = await estimateGas('color', { 
          x: pixel.x, 
          y: pixel.y, 
          color: pixel.color 
        });
        toast.loading(`Estimated gas: ${gasEstimate}`, { id: toastId });
        
        await setPixelColor(pixel.x, pixel.y, pixel.color);
      }

      toast.success('Successfully updated pixels!', { id: toastId });
      setPendingPixels([]);
    } catch (error) {
      console.error('Failed to submit pixels:', error);
      toast.error('Failed to update pixels', { id: toastId });
      
      // Reload canvas to show actual state
      await loadExistingPixels();
    } finally {
      setIsSubmitting(false);
    }
  };

  // Debounced submit function
  const debouncedSubmit = useCallback(
    debounce(() => {
      if (pendingPixels.length > 0) {
        submitPixels();
      }
    }, 2000),
    [pendingPixels]
  );

  // Auto-submit pending pixels after delay
  useEffect(() => {
    if (pendingPixels.length > 0 && !isSubmitting) {
      debouncedSubmit();
    }
    return () => {
      debouncedSubmit.cancel();
    };
  }, [pendingPixels, isSubmitting, debouncedSubmit]);

  // Show transaction status
  useEffect(() => {
    if (transactionStatus.color.error) {
      toast.error(`Color update failed: ${transactionStatus.color.error.message}`);
    }
  }, [transactionStatus]);

  return (
    <div className="relative">
      <canvas
        ref={canvasRef}
        onMouseDown={startDrawing}
        onMouseMove={draw}
        onMouseUp={stopDrawing}
        onMouseLeave={stopDrawing}
        className="border border-gray-300 cursor-crosshair"
      />
      <div className="absolute top-4 right-4 flex gap-2">
        <input
          type="color"
          value={currentColor}
          onChange={(e) => setCurrentColor(e.target.value)}
          className="w-8 h-8 cursor-pointer"
        />
        {pendingPixels.length > 0 && (
          <button
            onClick={() => submitPixels()}
            disabled={isSubmitting}
            className="px-4 py-2 bg-blue-500 text-white rounded disabled:opacity-50"
          >
            {isSubmitting ? 'Submitting...' : `Submit ${pendingPixels.length} Pixels`}
          </button>
        )}
      </div>
    </div>
  );
} 