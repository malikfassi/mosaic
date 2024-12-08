# Stargaze Pixel Canvas

A decentralized pixel canvas built on the Stargaze blockchain, inspired by the Million Dollar Homepage.

## Project Overview
Users can purchase, own, and modify pixels on a digital canvas. Each pixel can be customized with different colors, creating a dynamic and interactive NFT experience on the Stargaze chain.

## Repository Structure
```
├── contracts/           # Smart contract code
│   ├── pixel-canvas/    # Main contract implementation
│   └── tests/          # Contract tests
├── frontend/           # Web application
│   ├── components/     # React components
│   ├── hooks/         # Custom React hooks
│   └── pages/         # Next.js pages
├── scripts/           # Development and deployment scripts
└── docs/             # Project documentation
```

## Prerequisites
- Rust 1.69.0 or higher
- Node.js 18.x or higher
- cargo-generate
- Docker (for local testing)

## Quick Start

### Smart Contract Development
```bash
# Navigate to contracts directory
cd contracts/pixel-canvas

# Build the contract
cargo build

# Run tests
cargo test
```

### Frontend Development
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev
```

## Testing
- Unit tests: `cargo test`
- Integration tests: (Coming soon)
- Frontend tests: `npm test`

## Contributing
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
