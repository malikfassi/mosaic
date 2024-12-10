# Pixel Canvas

[![Frontend](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/frontend-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)
[![Coloring Contract](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/coloring-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)
[![NFT Contract](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/nft-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)

A decentralized pixel art canvas powered by Stargaze NFTs.

## Status

| Component | Status |
|-----------|--------|
| Frontend CI | ![Frontend](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/frontend-ci.json) |
| Coloring Contract CI | ![Coloring](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/coloring-ci.json) |
| NFT Contract CI | ![NFT](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/nft-ci.json) |
| Deploy Coloring | ![Deploy Coloring](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/deploy-coloring.json) |
| Deploy NFT | ![Deploy NFT](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/deploy-nft.json) |
| Coloring E2E | ![Coloring E2E](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/coloring-e2e.json) |
| NFT E2E | ![NFT E2E](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/nft-e2e.json) |
| Full E2E | ![Full E2E](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/${{ github.repository_owner }}/${{ secrets.BADGE_GIST_ID }}/raw/full-e2e.json) |

## Features

- 🎨 Draw pixels on a shared canvas
- 🔗 Each pixel is an NFT on Stargaze
- 🎭 Change pixel colors through the coloring contract
- 🌈 Full RGB color support
- ⚡ Real-time updates
- 🔒 Secure ownership verification

## Architecture

The project consists of three main components:

1. Frontend (Next.js)
   - Modern React application
   - WebGL-powered canvas rendering
   - Keplr wallet integration

2. NFT Contract (CosmWasm)
   - Handles NFT minting and transfers
   - Implements SG-721 standard
   - Manages pixel ownership

3. Coloring Contract (CosmWasm)
   - Controls pixel color changes
   - Verifies ownership
   - Maintains color history

## Development

### Prerequisites

- Node.js 18+
- Rust 1.70+
- Docker (optional)

### Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/pixel-canvas.git
   cd pixel-canvas
   ```

2. Install frontend dependencies:
   ```bash
   cd frontend
   npm install
   ```

3. Build contracts:
   ```bash
   cd contracts/pixel-nft
   cargo build
   ```

4. Set up environment variables:
   ```bash
   cp .env.example .env
   ```

### Running Locally

1. Start the frontend:
   ```bash
   cd frontend
   npm run dev
   ```

2. Deploy contracts to testnet:
   ```bash
   ./scripts/deploy.sh testnet
   ```

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
