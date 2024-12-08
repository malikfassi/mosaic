'use client';

import { FC, useEffect, useRef, useState } from 'react';
import { useCanvasWebSocket } from '@/hooks/useCanvasWebSocket';

interface PixelCanvasProps {
  width: number;
  height: number;
  pixelSize: number;
}

const PixelCanvas: FC<PixelCanvasProps> = ({ width, height, pixelSize }) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [selectedColor, setSelectedColor] = useState('#000000');
  const { isConnected } = useCanvasWebSocket(process.env.NEXT_PUBLIC_WS_URL || 'ws://localhost:3001');

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

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / pixelSize);
    const y = Math.floor((event.clientY - rect.top) / pixelSize);

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
        <input
          type="color"
          value={selectedColor}
          onChange={(e) => setSelectedColor(e.target.value)}
          className="w-8 h-8 cursor-pointer"
        />
        <div className="mt-2 text-sm text-gray-600">
          {isConnected ? 'Connected' : 'Disconnected'}
        </div>
      </div>
    </div>
  );
};

export default PixelCanvas; 