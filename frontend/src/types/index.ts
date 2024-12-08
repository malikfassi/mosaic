export interface Pixel {
  owner: string;
  color: string;
  lastUpdated: number;
}

export interface CanvasConfig {
  size: number;
  pixelPrice: string;
  owner: string;
}

export interface ContractInfo {
  address: string;
  creator: string;
  version: string;
}

export interface WalletInfo {
  address: string;
  balance: string;
  connected: boolean;
}

export interface PixelCoordinates {
  x: number;
  y: number;
} 