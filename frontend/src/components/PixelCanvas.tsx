'use client';

import { useEffect, useRef, useState, useCallback } from 'react';
import { toast } from 'react-hot-toast';
import { useContract } from '@/hooks/useContract';
import type { Pixel, PixelCoordinates } from '@/types';

interface CanvasState {
  pixels: Map<string, Pixel>;
  loading: boolean;
  error: string | null;
  selectedPixel: PixelCoordinates | null;
  selectedColor: string;
}

const CANVAS_SIZE = 100;
const PIXEL_SIZE = 5;
const DEFAULT_COLOR = '#FFFFFF';

export default function PixelCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [state, setState] = useState<CanvasState>({
    pixels: new Map(),
    loading: true,
    error: null,
    selectedPixel: null,
    selectedColor: DEFAULT_COLOR,
  });

  const { buyPixel, setPixelColor, getCanvas } = useContract();

  const drawCanvas = useCallback(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#000000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    ctx.strokeStyle = '#1a1a1a';
    ctx.lineWidth = 0.5;
    for (let x = 0; x <= canvas.width; x += PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, canvas.height);
      ctx.stroke();
    }
    for (let y = 0; y <= canvas.height; y += PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(canvas.width, y);
      ctx.stroke();
    }

    // Draw pixels
    state.pixels.forEach((pixel, key) => {
      const [x, y] = key.split(',').map(Number);
      ctx.fillStyle = pixel.color;
      ctx.fillRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
    });

    // Draw selected pixel
    if (state.selectedPixel) {
      ctx.strokeStyle = '#FFFFFF';
      ctx.lineWidth = 2;
      ctx.strokeRect(
        state.selectedPixel.x * PIXEL_SIZE,
        state.selectedPixel.y * PIXEL_SIZE,
        PIXEL_SIZE,
        PIXEL_SIZE
      );
    }
  }, [state.pixels, state.selectedPixel]);

  // Load canvas data
  useEffect(() => {
    const loadCanvas = async () => {
      try {
        setState(prev => ({ ...prev, loading: true, error: null }));
        const canvasData = await getCanvas();
        
        const pixelMap = new Map();
        canvasData.forEach(([x, y, pixel]: [number, number, Pixel]) => {
          pixelMap.set(`${x},${y}`, pixel);
        });

        setState(prev => ({
          ...prev,
          pixels: pixelMap,
          loading: false,
        }));
      } catch (error) {
        console.error('Error loading canvas:', error);
        setState(prev => ({
          ...prev,
          loading: false,
          error: 'Failed to load canvas data',
        }));
        toast.error('Failed to load canvas data');
      }
    };

    loadCanvas();
  }, [getCanvas]);

  // Initialize canvas size
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.width = CANVAS_SIZE * PIXEL_SIZE;
    canvas.height = CANVAS_SIZE * PIXEL_SIZE;
    drawCanvas();
  }, [drawCanvas]);

  // Handle canvas click
  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / PIXEL_SIZE);
    const y = Math.floor((event.clientY - rect.top) / PIXEL_SIZE);

    if (x >= 0 && x < CANVAS_SIZE && y >= 0 && y < CANVAS_SIZE) {
      setState(prev => ({ ...prev, selectedPixel: { x, y } }));
    }
  };

  // Handle buy pixel
  const handleBuyPixel = async () => {
    if (!state.selectedPixel) return;

    try {
      setState(prev => ({ ...prev, loading: true }));
      await buyPixel(state.selectedPixel.x, state.selectedPixel.y);
      toast.success('Pixel purchased successfully!');
      
      // Refresh canvas
      const canvasData = await getCanvas();
      const pixelMap = new Map();
      canvasData.forEach(([x, y, pixel]: [number, number, Pixel]) => {
        pixelMap.set(`${x},${y}`, pixel);
      });
      setState(prev => ({ ...prev, pixels: pixelMap }));
    } catch (error) {
      console.error('Error buying pixel:', error);
      toast.error('Failed to buy pixel');
    } finally {
      setState(prev => ({ ...prev, loading: false }));
    }
  };

  // Handle set pixel color
  const handleSetPixelColor = async () => {
    if (!state.selectedPixel) return;

    try {
      setState(prev => ({ ...prev, loading: true }));
      await setPixelColor(
        state.selectedPixel.x,
        state.selectedPixel.y,
        state.selectedColor
      );
      toast.success('Pixel color updated successfully!');
      
      // Refresh canvas
      const canvasData = await getCanvas();
      const pixelMap = new Map();
      canvasData.forEach(([x, y, pixel]: [number, number, Pixel]) => {
        pixelMap.set(`${x},${y}`, pixel);
      });
      setState(prev => ({ ...prev, pixels: pixelMap }));
    } catch (error) {
      console.error('Error setting pixel color:', error);
      toast.error('Failed to set pixel color');
    } finally {
      setState(prev => ({ ...prev, loading: false }));
    }
  };

  if (state.error) {
    return (
      <div className="text-center p-4 bg-red-500/10 rounded-lg">
        <p className="text-red-500">{state.error}</p>
        <button
          onClick={() => window.location.reload()}
          className="mt-4 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
        >
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="flex flex-col items-center space-y-6">
      <div className="relative">
        {state.loading && (
          <div className="absolute inset-0 bg-black/50 flex items-center justify-center">
            <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-white"></div>
          </div>
        )}
        <canvas
          ref={canvasRef}
          onClick={handleCanvasClick}
          className="border border-gray-800 rounded-lg cursor-pointer"
        />
      </div>
      
      <div className="flex items-center space-x-4">
        <input
          type="color"
          value={state.selectedColor}
          onChange={(e) => setState(prev => ({ ...prev, selectedColor: e.target.value }))}
          className="w-12 h-12 rounded cursor-pointer"
        />
        
        <button
          onClick={handleBuyPixel}
          disabled={!state.selectedPixel || state.loading}
          className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Buy Pixel
        </button>
        
        <button
          onClick={handleSetPixelColor}
          disabled={!state.selectedPixel || state.loading}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          Set Color
        </button>
      </div>
      
      {state.selectedPixel && (
        <div className="text-sm text-gray-400">
          Selected: ({state.selectedPixel.x}, {state.selectedPixel.y})
          {state.pixels.has(`${state.selectedPixel.x},${state.selectedPixel.y}`) && (
            <span className="ml-2">
              - Owned by: {state.pixels.get(`${state.selectedPixel.x},${state.selectedPixel.y}`)?.owner.slice(0, 8)}...
            </span>
          )}
        </div>
      )}
    </div>
  );
} 