import { RenderResult } from '@testing-library/react';
import { ReactElement } from 'react';

export interface CustomRenderOptions {
  preloadedState?: Record<string, unknown>;
  renderOptions?: Parameters<typeof render>[1];
}

export interface CustomRenderResult extends RenderResult {
  rerender: (ui: ReactElement) => void;
  unmount: () => void;
} 