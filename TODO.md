# Project TODO List

## 🚨 Immediate Architecture Priority
1. Contract Reorganization
   - [ ] Code Cleanup:
     - [ ] Move coloring functionality from pixel-canvas to pixel-nft/contracts/coloring/
     - [ ] Remove redundant code from pixel-nft/src/
     - [ ] Clean up duplicate functionality
   - [ ] Contract Structure:
     - [ ] Finalize sg721-pixel contract
     - [ ] Set up new coloring contract
     - [ ] Update contract dependencies
   - [ ] Integration:
     - [ ] Update contract imports
     - [ ] Set up cross-contract communication
     - [ ] Update tests to reflect new structure

## Immediate Priority (MVP & Testing)
1. Testing & Automation
   - [ ] Contract Testing
     - [ ] Unit tests for all contract functions
     - [ ] Integration tests with mock chain
     - [ ] Property-based testing
     - [ ] Coverage reports
   - [ ] Frontend Testing
     - [ ] Component unit tests
     - [ ] Integration tests
     - [ ] E2E tests with Cypress
   - [ ] CI/CD Pipeline
     - [ ] GitHub Actions setup
     - [ ] Automated testing
     - [ ] Automated deployment
     - [ ] Code quality checks
     - [ ] Coverage reports

2. MVP Preparation
   - [ ] Core Contract Features
     - [ ] Complete pixel buying functionality
     - [ ] Complete pixel color setting
     - [ ] Basic ownership validation
     - [ ] Error handling improvements
   - [ ] Frontend Stability
     - [ ] Error boundary implementation
     - [ ] Loading states
     - [ ] Transaction feedback
     - [ ] Connection stability
   - [ ] Transaction Handling
     - [ ] Batch transaction support
     - [ ] Transaction preview
     - [ ] Gas estimation
     - [ ] Transaction history
     - [ ] Retry mechanism
   - [ ] UI/UX Improvements
     - [ ] Canvas zoom and pan
     - [ ] Color palette presets
     - [ ] Pixel ownership hover
     - [ ] Undo/Redo functionality
     - [ ] Mobile responsiveness
   - [ ] Loading States & Animations
     - [ ] Pixel placement animation
     - [ ] Transaction progress indicator
     - [ ] Loading skeletons
     - [ ] Success/Error animations
     - [ ] Network status indicator
   - [ ] Documentation
     - [ ] Setup instructions
     - [ ] Testing guide
     - [ ] API documentation

3. Manual Testing Phase
   - [ ] Contract Testing Scenarios
     - [ ] Pixel purchase flow
     - [ ] Color setting flow
     - [ ] Error cases
     - [ ] Edge cases
   - [ ] Frontend Testing Scenarios
     - [ ] Wallet connection
     - [ ] Transaction flow
     - [ ] UI responsiveness
     - [ ] Error handling

## Contract Architecture Updates
1. NFT Contract (sg721-base)
   - [ ] Setup base contract
     - [ ] Initialize sg721-base template
     - [ ] Configure metadata schema
     - [ ] Add pixel-specific attributes
   - [ ] Implement extensions
     - [ ] Add marketplace functionality
     - [ ] Add ownership validation
     - [ ] Add transfer restrictions
   - [ ] Testing
     - [ ] Unit tests
     - [ ] Integration tests
     - [ ] Property-based tests

2. Coloring Contract
   - [ ] Setup contract
     - [ ] State management
     - [ ] Message types
     - [ ] Error handling
   - [ ] Core functionality
     - [ ] Color change permission system
     - [ ] Fee collection
     - [ ] Time-based restrictions
   - [ ] Testing
     - [ ] Unit tests
     - [ ] Integration tests
     - [ ] Property-based tests

3. Contract Integration
   - [ ] Cross-contract communication
     - [ ] NFT ownership validation
     - [ ] Color change permissions
     - [ ] Fee distribution
   - [ ] Testing
     - [ ] End-to-end tests
     - [ ] Multi-user scenarios
     - [ ] Error cases

## Future Enhancements (Post-MVP)
1. Stargaze Integration
   - [ ] Name Resolution
     - [ ] Add name lookup support
     - [ ] Reverse lookup (address to name)
     - [ ] Cache name resolutions
     - [ ] Update UI to show names
   - [ ] Contract Features
     - [ ] Implement rental system
     - [ ] Time-based ownership
     - [ ] Rental marketplace
     - [ ] Revenue sharing

2. Real-time Updates
   - [ ] WebSocket Integration
     - [ ] Set up WebSocket server
     - [ ] Implement connection handling
     - [ ] Add authentication
     - [ ] Add rate limiting
   - [ ] Canvas Sync
     - [ ] Real-time pixel updates
     - [ ] User presence indicators
     - [ ] Live transaction status
     - [ ] Connection status indicator
   - [ ] Performance Optimization
     - [ ] Message batching
     - [ ] Reconnection strategy
     - [ ] State reconciliation
     - [ ] Conflict resolution

3. Analytics & Dashboard
   - [ ] User Dashboard
     - [ ] Owned pixels overview
     - [ ] Rental management
     - [ ] Transaction history
     - [ ] Revenue tracking
   - [ ] Analytics
     - [ ] Pixel activity heatmap
     - [ ] Price history
     - [ ] User engagement metrics
     - [ ] Revenue analytics

## Build Environment
- [x] Fix dependency issues
  - [x] Update to Stargaze recommended versions
  - [x] Remove nightly requirements
  - [x] Fix build errors
- [ ] Development setup
  - [ ] Local testnet configuration
  - [ ] Deployment scripts
  - [ ] Test environment

## Infrastructure & Setup Requirements
1. Environment Variables Setup
   - [ ] Create root `.env`:
     - [ ] STARGAZE_TESTNET_RPC
     - [ ] STARGAZE_TESTNET_CHAIN_ID
     - [ ] DEPLOYMENT_WALLET_MNEMONIC
     - [ ] DEPLOYMENT_WALLET_ADDRESS
   - [ ] Create frontend `.env`:
     - [ ] NEXT_PUBLIC_BASE_URL
     - [ ] NEXT_PUBLIC_STARGAZE_RPC
     - [ ] NEXT_PUBLIC_STARGAZE_REST
     - [ ] NEXT_PUBLIC_STARGAZE_CHAIN_ID
     - [ ] NEXT_PUBLIC_CONTRACT_ADDRESS
   - [ ] Create `.env.example` templates

2. GitHub Repository Configuration
   - [ ] Enable GitHub Actions
   - [ ] Set up repository secrets:
     - [ ] DEPLOYMENT_WALLET_MNEMONIC
     - [ ] DEPLOYMENT_WALLET_ADDRESS
     - [ ] CODECOV_TOKEN
   - [ ] Configure branch protection rules:
     - [ ] Require pull request reviews
     - [ ] Require status checks
     - [ ] Enforce linear history

3. Stargaze Network Setup
   - [ ] Create and fund testnet wallet
   - [ ] Obtain testnet STARS (~100 minimum)
   - [ ] Set up mainnet wallet
   - [ ] Verify contract deployment permissions

## Contract Implementation Progress

### ✅ Base Structure (Completed)
- [x] Contract file structure setup
- [x] Base dependencies configuration
- [x] State management structures
- [x] Message types definition
- [x] Error handling
- [x] Metadata structure
- [x] Entry points setup

### 🚧 Core Implementation (In Progress)
- [ ] Execute Functions:
  - [x] Mint pixel NFT
  - [x] Transfer pixel
  - [x] List pixel for sale
  - [x] Buy listed pixel
  - [x] Set pixel color
  - [x] Update configuration
  - [ ] Unlist pixel
- [ ] Query Functions:
  - [x] Get pixel by token ID
  - [x] Get pixel by coordinates
  - [x] Get pixels by owner
  - [x] Get canvas state
  - [x] Get listed pixels

### 🧪 Testing Suite (In Progress)
- [x] Basic Tests:
  - [x] Mock environment setup
  - [x] Instantiation tests
  - [x] Basic pixel operations
- [x] Execute Function Tests:
  - [x] Mint pixel tests
  - [x] Transfer tests
  - [x] Marketplace tests
  - [x] Color change tests
  - [x] Admin tests
- [x] Query Function Tests:
  - [x] Single Pixel Queries
  - [x] Collection Queries
  - [x] Canvas State
  - [x] Error Cases
