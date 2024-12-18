'use client';

import { useEffect, useRef, useState } from 'react';
import { useContract } from '@/hooks/useContract';
import { useToast } from '@/components/ui/use-toast';
import { Button } from '@/components/ui/button';
import { ColorPicker } from '@/components/ColorPicker';

interface Pixel {
  pixel_id: number;
  color: string;
}

const CANVAS_SIZE = 1000;
const CHUNK_SIZE = 100;
const PIXEL_SIZE = 10;

export function PixelCanvas() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [selectedColor, setSelectedColor] = useState('#000000');
  const [selectedPixel, setSelectedPixel] = useState<Pixel | null>(null);
  const [isDrawing, setIsDrawing] = useState(false);
  const { isConnected, client } = useContract();
  const { toast } = useToast();

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    canvas.width = CANVAS_SIZE;
    canvas.height = CANVAS_SIZE;

    // Draw grid
    ctx.strokeStyle = '#e5e7eb';
    ctx.lineWidth = 1;

    for (let x = 0; x <= CANVAS_SIZE; x += PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, CANVAS_SIZE);
      ctx.stroke();
    }

    for (let y = 0; y <= CANVAS_SIZE; y += PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(CANVAS_SIZE, y);
      ctx.stroke();
    }

    // Draw chunk borders
    ctx.strokeStyle = '#9ca3af';
    ctx.lineWidth = 2;

    for (let x = 0; x <= CANVAS_SIZE; x += CHUNK_SIZE * PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(x, 0);
      ctx.lineTo(x, CANVAS_SIZE);
      ctx.stroke();
    }

    for (let y = 0; y <= CANVAS_SIZE; y += CHUNK_SIZE * PIXEL_SIZE) {
      ctx.beginPath();
      ctx.moveTo(0, y);
      ctx.lineTo(CANVAS_SIZE, y);
      ctx.stroke();
    }
  }, []);

  const handleCanvasClick = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isConnected || !client) {
      toast({
        title: 'Not Connected',
        description: 'Please connect your wallet to draw pixels.',
        variant: 'destructive',
      });
      return;
    }

    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / PIXEL_SIZE);
    const y = Math.floor((event.clientY - rect.top) / PIXEL_SIZE);

    if (x < 0 || x >= CANVAS_SIZE / PIXEL_SIZE || y < 0 || y >= CANVAS_SIZE / PIXEL_SIZE) {
      return;
    }

    // Convert x,y to pixel_id
    const pixel_id = y * (CANVAS_SIZE / PIXEL_SIZE) + x;
    setSelectedPixel({ pixel_id, color: selectedColor });
    setIsDrawing(true);
  };

  const handleMouseMove = (event: React.MouseEvent<HTMLCanvasElement>) => {
    if (!isDrawing || !isConnected || !client) return;

    const canvas = canvasRef.current;
    if (!canvas) return;

    const rect = canvas.getBoundingClientRect();
    const x = Math.floor((event.clientX - rect.left) / PIXEL_SIZE);
    const y = Math.floor((event.clientY - rect.top) / PIXEL_SIZE);

    if (x < 0 || x >= CANVAS_SIZE / PIXEL_SIZE || y < 0 || y >= CANVAS_SIZE / PIXEL_SIZE) {
      return;
    }

    // Convert x,y to pixel_id
    const pixel_id = y * (CANVAS_SIZE / PIXEL_SIZE) + x;
    setSelectedPixel({ pixel_id, color: selectedColor });
  };

  const handleMouseUp = () => {
    setIsDrawing(false);
  };

  const handleColorChange = (color: string) => {
    setSelectedColor(color);
  };

  const drawPixel = async () => {
    if (!selectedPixel || !isConnected || !client) return;

    try {
      // TODO: Implement contract interaction
      toast({
        title: 'Pixel Drawn',
        description: `Drew pixel at (${selectedPixel.pixel_id}) with color ${selectedPixel.color}`,
      });
    } catch (error) {
      toast({
        title: 'Error',
        description: error instanceof Error ? error.message : 'Failed to draw pixel',
        variant: 'destructive',
      });
    }
  };

  return (
    <div className="flex flex-col items-center gap-4">
      <div className="flex items-center gap-4">
        <ColorPicker value={selectedColor} onChange={handleColorChange} />
        <Button
          onClick={drawPixel}
          disabled={!selectedPixel || !isConnected}
        >
          Draw Pixel
        </Button>
      </div>
      <canvas
        ref={canvasRef}
        className="border border-gray-300 cursor-crosshair"
        onClick={handleCanvasClick}
        onMouseMove={handleMouseMove}
        onMouseUp={handleMouseUp}
        onMouseLeave={handleMouseUp}
      />
    </div>
  );
} 