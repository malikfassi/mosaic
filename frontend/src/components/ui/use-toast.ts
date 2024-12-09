import { useState, useCallback } from 'react';

interface ToastProps {
  title?: string;
  description?: string;
  variant?: 'default' | 'destructive';
}

interface Toast extends ToastProps {
  id: string;
}

export function useToast() {
  const [toasts, setToasts] = useState<Toast[]>([]);

  const toast = useCallback(({ title, description, variant = 'default' }: ToastProps) => {
    const id = Math.random().toString(36).substring(7);
    const newToast: Toast = {
      id,
      title,
      description,
      variant,
    };

    setToasts((prev) => [...prev, newToast]);

    // Auto dismiss after 5 seconds
    setTimeout(() => {
      setToasts((prev) => prev.filter((t) => t.id !== id));
    }, 5000);
  }, []);

  return { toast, toasts };
} 