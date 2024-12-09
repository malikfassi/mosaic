# Stargaze Pixel Canvas

A decentralized 100-million pixel canvas (10,000 x 10,000) built on the Stargaze blockchain, inspired by the Million Dollar Homepage.

## Project Overview
Users can purchase, own, and modify 10x10 pixel areas on a massive digital canvas. Each area is an NFT that can be traded on the Stargaze marketplace, while individual pixels within owned areas can be customized with different colors or rented out for temporary use.

## Architecture
The project uses a chunked architecture for scalability:
- Area-based NFTs (10x10 pixels each)
- Chunked color management (100x100 pixels per chunk)
- WebGL-accelerated rendering
- Progressive loading with Level of Detail (LOD)

## Repository Structure
```
├── contracts/           # Smart contract code
│   ├── sg721-area/     # Area-based NFT contract
│   ├── coloring/       # Chunked coloring contract
│   └── tests/          # Contract tests
├── frontend/           # Web application
│   ├── components/     # React components
│   │   └── canvas/    # WebGL canvas components
│   ├── hooks/         # Custom React hooks
│   │   └── chunk/     # Chunk management hooks
│   └── pages/         # Next.js pages
├── scripts/           # Development and deployment scripts
└── docs/             # Project documentation
```

## Prerequisites
- Rust 1.69.0 or higher
- Node.js 18.x or higher
- cargo-generate
- Docker (for local testing)
- GPU with WebGL 2.0 support

## Development Standards
### Smart Contract
- CosmWasm 2.2.0-rc.3
- Optimized state management
- Chunked data storage
- All functions must have unit tests
- Coverage requirement: 90%+

### Frontend
- Next.js 14.0.3
- React 18.3.1
- WebGL 2.0
- Three.js for rendering
- TypeScript strict mode enabled
- Test coverage requirements:
  - Components: 85%+
  - Hooks: 90%+
  - Utils: 95%+

### Performance Requirements
- Initial load < 2s
- Chunk load < 100ms
- Render at 60fps
- Memory usage < 100MB
- Cache size < 50MB

### Testing Standards
- Unit tests required for all new code
- Integration tests for critical paths
- Performance benchmarks
- Memory leak checks
- WebGL conformance tests

### Code Quality
- ESLint configuration enforced
- Prettier for code formatting
- TypeScript strict mode
- No warnings policy
- Regular dependency updates
- Performance profiling

### Development Workflow
1. Before starting work:
   ```bash
   git pull
   ./scripts/validate.sh
   ```

2. During development:
   ```bash
   # Run validation frequently
   ./scripts/validate.sh
   
   # Check test coverage
   cd frontend && npm run test
   cd ../contracts && cargo test
   
   # Run performance tests
   npm run bench
   ```

3. Before committing:
   ```bash
   # Validation runs automatically via pre-commit hook
   git add .
   git commit -m "type(scope): description"
   ```

## Quick Start

### Smart Contract Development
```bash
# Navigate to contracts directory
cd contracts

# Build the contracts
cargo build

# Run tests with benchmarks
cargo test --features "benchmark"
```

### Frontend Development
```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install

# Start development server
npm run dev

# Run WebGL tests
npm run test:webgl
```

## Testing
- Unit tests: `cargo test`
- Integration tests: `cargo test --features "integration"`
- Frontend tests: `npm test`
- Performance tests: `npm run bench`
- WebGL tests: `npm run test:webgl`

## Contributing
Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
