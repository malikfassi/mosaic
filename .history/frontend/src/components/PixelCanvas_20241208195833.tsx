'use client';

import { useEffect, useRef, useState } from 'react';
import { toast } from 'react-hot-toast';

interface Pixel {
  owner: string;
  color: string;
  lastUpdated: number;
}

export default function PixelCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [selectedColor, setSelectedColor] = useState('#000000');
  const [selectedPixel, setSelectedPixel] = useState<{ x: number; y: number } | null>(null);
  const [loading, setLoading] = useState(false);
  const [pixels, setPixels] = useState<Map<string, Pixel>>(new Map());

  const CANVAS_SIZE = 100; // This should match the contract's canvas size
  const PIXEL_SIZE = 5; // Size of each pixel in the canvas

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    canvas.width = CANVAS_SIZE * PIXEL_SIZE;
    canvas.height = CANVAS_SIZE * PIXEL_SIZE;

    // Initial render
    drawCanvas();
  }, []);

  const drawCanvas = () => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Clear canvas
    ctx.fillStyle = '#FFFFFF';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw grid
    ctx.strokeStyle = '#EEEEEE';
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
    pixels.forEach((pixel, key) => {
      const [x, y] = key.split(',').map(Number);
      ctx.fillStyle = pixel.color;
      ctx.fillRect(x * PIXEL_SIZE, y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
    });

    // Draw selected pixel
    if (selectedPixel) {
      ctx.strokeStyle = '#FF0000';
      ctx.lineWidth = 2;
      ctx.strokeRect(
        selectedPixel.x * PIXEL_SIZE,
        selectedPixel.y * PIXEL_SIZE,
        PIXEL_SIZE,
        PIXEL_SIZE
      );
    }
  };

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / PIXEL_SIZE);
    const y = Math.floor((event.clientY - rect.top) / PIXEL_SIZE);

    if (x >= 0 && x < CANVAS_SIZE && y >= 0 && y < CANVAS_SIZE) {
      setSelectedPixel({ x, y });
    }
  };

  const buyPixel = async () => {
    if (!selectedPixel) return;

    try {
      setLoading(true);
      // TODO: Implement contract interaction
      toast.success('Pixel purchased successfully!');
    } catch (error) {
      console.error('Error buying pixel:', error);
      toast.error('Failed to buy pixel');
    } finally {
      setLoading(false);
    }
  };

  const setPixelColor = async () => {
    if (!selectedPixel) return;

    try {
      setLoading(true);
      // TODO: Implement contract interaction
      toast.success('Pixel color updated successfully!');
    } catch (error) {
      console.error('Error setting pixel color:', error);
      toast.error('Failed to set pixel color');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    drawCanvas();
  }, [pixels, selectedPixel]);

  return (
    <div className="flex flex-col items-center">
      <canvas
        ref={canvasRef}
        onClick={handleCanvasClick}
        className="border border-gray-300 cursor-pointer"
      />
      
      <div className="mt-4 flex gap-4">
        <input
          type="color"
          value={selectedColor}
          onChange={(e) => setSelectedColor(e.target.value)}
          className="w-12 h-12"
        />
        
        <button
          onClick={buyPixel}
          disabled={!selectedPixel || loading}
          className="bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
        >
          Buy Pixel
        </button>
        
        <button
          onClick={setPixelColor}
          disabled={!selectedPixel || loading}
          className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:opacity-50"
        >
          Set Color
        </button>
      </div>
      
      {selectedPixel && (
        <div className="mt-4 text-sm">
          Selected: ({selectedPixel.x}, {selectedPixel.y})
        </div>
      )}
    </div>
  );
} 