# Pixel Canvas

[![Frontend CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/frontend-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)
[![Mosaic Tile CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-tile-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)
[![Mosaic Vending CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-vending-ci.json)](https://github.com/malikfassi/mosaic/actions/workflows/pixel-canvas.yml)

A decentralized pixel art canvas powered by Stargaze NFTs.

## Status

| Component | Status |
|-----------|--------|
| Frontend CI | ![Frontend CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/frontend-ci.json) |
| Mosaic Tile CI | ![Mosaic Tile CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-tile-ci.json) |
| Mosaic Vending CI | ![Mosaic Vending CI](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-vending-ci.json) |
| Deploy Mosaic Tile | ![Deploy Mosaic Tile](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/deploy-mosaic-tile.json) |
| Deploy Mosaic Vending | ![Deploy Mosaic Vending](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/deploy-mosaic-vending.json) |
| Mosaic Tile E2E | ![Mosaic Tile E2E](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-tile-e2e.json) |
| Mosaic Vending E2E | ![Mosaic Vending E2E](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/mosaic-vending-e2e.json) |
| Full E2E Tests | ![Full E2E Tests](https://img.shields.io/endpoint?url=https://gist.githubusercontent.com/malikfassi/1ff46a4915f58fa0fce5cab7577f94f1/raw/full-e2e.json) |

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

2. Mosaic Tile NFT Contract (CosmWasm)
   - Handles NFT minting and transfers
   - Implements SG-721 standard
   - Manages pixel ownership

3. Mosaic Vending Contract (CosmWasm)
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
   git clone https://github.com/malikfassi/mosaic.git
   cd mosaic
   ```

2. Install frontend dependencies:
   ```bash
   cd frontend
   npm install
   ```

3. Build contracts:
   ```bash
   cd contracts/mosaic_tile_nft
   cargo build
   cargo run --example schema
   cd ../mosaic_vending_minter
   cargo build
   cargo run --example schema
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

### Testing

The project includes comprehensive test coverage:

1. Unit Tests
   ```bash
   # Frontend tests
   cd frontend && npm test
   
   # Contract tests
   cd contracts/mosaic_tile_nft && cargo test
   cd contracts/mosaic_vending_minter && cargo test
   ```

2. E2E Tests
   ```bash
   # Run full E2E test suite
   cd e2e && npm test
   ```

### CI/CD Pipeline

Our CI/CD pipeline ensures code quality and reliability:

1. Build & Test
   - Frontend linting and tests
   - Contract formatting, clippy checks, and tests
   - Schema generation validation

2. Deployment
   - Automated contract deployment
   - Environment configuration
   - Security checks

3. E2E Testing
   - Individual contract E2E tests
   - Full system integration tests

### Security

- Regular security audits
- See [SECURITY.md](SECURITY.md) for details

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.
