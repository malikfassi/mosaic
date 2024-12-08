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

## Development Standards
### Smart Contract
- CosmWasm 2.2.0-rc.3
- Use `to_json_binary` instead of deprecated `to_binary`
- All functions must have unit tests
- Coverage requirement: 90%+

### Frontend
- Next.js 14.0.3 (15.0.4 upgrade planned)
- React 18.3.1 (19.0.0 upgrade planned)
- @cosmjs/cosmwasm-stargate 0.31.3
- TypeScript strict mode enabled
- Test coverage requirements:
  - Components: 85%+
  - Hooks: 90%+
  - Utils: 95%+

### Testing Standards
- Unit tests required for all new code
- Integration tests for critical paths
- E2E tests for main user flows
- Test coverage tracked in CI/CD

### Code Quality
- ESLint configuration enforced
- Prettier for code formatting
- TypeScript strict mode
- No warnings policy
- Regular dependency updates

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
   cd ../contracts/pixel-canvas && cargo test
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
