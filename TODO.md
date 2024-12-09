# Project TODO List

## 🚨 Phase 1: Core Functionality (Q1 2024)

### Contract Architecture
- [x] Base Structure
  - [x] Contract file structure setup
  - [x] Base dependencies configuration
  - [x] State management structures
  - [x] Message types definition
  - [x] Error handling
  - [x] Metadata structure

### NFT Contract (sg721-area)
- [ ] Core Implementation
  - [ ] Area-based NFT structure (10x10)
  - [ ] Area metadata schema
  - [ ] Ownership validation
  - [ ] Transfer restrictions
  - [ ] Stargaze marketplace integration
  - [ ] Area merging/splitting logic
  - [ ] Non-minted Area Flow:
    - [ ] Redirect to Stargaze marketplace
    - [ ] Clear status indicators
    - [ ] Price information display
    - [ ] Ownership vs. Coloring explanation

### Coloring Contract
- [ ] Core Implementation
  - [ ] Chunked state management (100x100)
  - [ ] Sparse color storage
  - [ ] Color rental system
  - [ ] Time-based permissions
  - [ ] Fee collection system
  - [ ] Cross-contract communication
  - [ ] Non-minted Interaction:
    - [ ] Preview-only mode
    - [ ] Clear ownership requirements
    - [ ] Marketplace redirection
    - [ ] Action comparison info

### Frontend MVP
- [ ] WebGL Canvas
  - [ ] Chunk manager implementation
  - [ ] WebGL renderer setup
  - [ ] Texture management
  - [ ] Level of Detail system
  - [ ] Progressive loading
  - [ ] Cache management

- [ ] Non-minted Area UX
  - [ ] "Buy on Stargaze" flow
  - [ ] Visual distinction for non-minted areas
  - [ ] Interactive tooltips explaining:
    - [ ] Ownership vs. coloring rights
    - [ ] Minting process
    - [ ] Price information
    - [ ] Expected capabilities
  - [ ] Seamless marketplace redirection
  - [ ] Status indicators

- [ ] User Interface
  - [ ] Area selection tools
  - [ ] Color picker with history
  - [ ] Chunk loading indicators
  - [ ] Transaction status display
  - [ ] Mobile-responsive design

- [ ] Performance Optimization
  - [ ] Chunk preloading
  - [ ] WebGL state optimization
  - [ ] Memory management
  - [ ] Background worker setup
  - [ ] Cache invalidation

### User Education
- [ ] Documentation
  - [ ] Clear ownership model
  - [ ] Minting vs. coloring explanation
  - [ ] Stargaze integration guide
  - [ ] Price structure
  - [ ] Feature comparison table

### Testing & Quality
- [ ] Contract Testing
  - [ ] Area NFT unit tests
  - [ ] Chunk management tests
  - [ ] Color rental tests
  - [ ] Integration tests
  - [ ] Performance benchmarks
  - [ ] Non-minted Tests:
    - [ ] Marketplace redirection
    - [ ] Status checks
    - [ ] Permission validation
    - [ ] Integration tests

- [ ] Frontend Testing
  - [ ] WebGL renderer tests
  - [ ] Chunk manager tests
  - [ ] Component tests
  - [ ] Integration tests
  - [ ] Performance profiling

- [ ] CI/CD Pipeline
  - [ ] WebGL test automation
  - [ ] Performance benchmark tracking
  - [ ] Memory leak detection
  - [ ] Bundle size monitoring

## 🚀 Phase 2: Enhanced Features (Q2 2024)

### Image Upload System
- [ ] Core Implementation
  - [ ] Chunked image processing
  - [ ] Area-based preview
  - [ ] Color optimization
  - [ ] Cost estimation
  - [ ] Batch transaction planning

### Rental System
- [ ] Features
  - [ ] Area-based rental logic
  - [ ] Time-based pricing
  - [ ] Chunk-aware operations
  - [ ] Renewal system
  - [ ] Conflict resolution

### User Dashboard
- [ ] Implementation
  - [ ] Area ownership overview
  - [ ] Chunk-based color history
  - [ ] Transaction history
  - [ ] Rental management

## 🎯 Phase 3: Advanced Features (Q3 2024)

### Brand Integration
- [ ] Features
  - [ ] Multi-area reservation
  - [ ] Chunk-aware waiting list
  - [ ] Priority system
  - [ ] Bulk operations

### Price Discovery
- [ ] Implementation
  - [ ] Area-based pricing
  - [ ] Location value calculation
  - [ ] Demand tracking
  - [ ] Dynamic adjustments

### Analytics
- [ ] Features
  - [ ] Chunk activity tracking
  - [ ] Area value analytics
  - [ ] Usage patterns
  - [ ] Performance metrics

## 🛠 Infrastructure Requirements

### Environment Setup
- [ ] Development
  - [ ] Local testnet with chunking
  - [ ] WebGL development tools
  - [ ] Performance monitoring
  - [ ] Memory profiling

### Network Setup
- [ ] Testnet
  - [ ] Area contract deployment
  - [ ] Chunk contract deployment
  - [ ] Integration testing
  - [ ] Performance testing

### Security
- [ ] Implementation
  - [ ] Area access control
  - [ ] Chunk validation
  - [ ] Rate limiting
  - [ ] Emergency procedures

## Immediate Next Steps
1. [ ] Implement chunk manager
   - [ ] Create ChunkManager class
   - [ ] Add LRU cache
   - [ ] Implement loading queue
   - [ ] Add background preloading

2. [ ] Set up WebGL renderer
   - [ ] Create shader programs
   - [ ] Implement texture management
   - [ ] Add chunk rendering
   - [ ] Optimize state changes

3. [ ] Update area contract
   - [ ] Add area-based storage
   - [ ] Implement merging logic
   - [ ] Add ownership validation
   - [ ] Update tests
   - [ ] Non-minted handling:
     - [ ] Marketplace integration
     - [ ] Status tracking
     - [ ] Permission checks
     - [ ] Redirection logic

4. [ ] Update coloring contract
   - [ ] Add chunk-based storage
   - [ ] Implement sparse arrays
   - [ ] Add rental logic
   - [ ] Update tests
   - [ ] Non-minted handling:
     - [ ] Preview mode
     - [ ] Permission checks
     - [ ] Status indicators
     - [ ] Marketplace hooks

5. [ ] User Experience Flow
   - [ ] Design marketplace redirection
   - [ ] Create educational tooltips
   - [ ] Implement status indicators
   - [ ] Add feature comparison
   - [ ] Create onboarding guide

## Key Differences Documentation

### Minted vs. Non-minted Areas
- [ ] Implementation
  - [ ] Clear ownership model
  - [ ] Permission structure
  - [ ] Available actions
  - [ ] Cost implications
  - [ ] Time considerations

### Action Comparison Table
- [ ] Features
  - [ ] Color changes
  - [ ] Rental options
  - [ ] Transfer rights
  - [ ] Income potential
  - [ ] Future capabilities

### User Flow Documentation
- [ ] Core Paths
  - [ ] Direct coloring (minted)
  - [ ] Marketplace purchase flow
  - [ ] Permission acquisition
  - [ ] Status transitions
  - [ ] Action restrictions

## 🧹 Immediate Cleanup (Priority)
- [ ] GitHub Actions Cleanup
  - [ ] Remove duplicate workflow files:
    - [ ] `deploy.yml` (replaced by contract-specific deployments)
    - [ ] `test.yml` (replaced by e2e-tests)
    - [ ] `deployment.yml` (obsolete)
    - [ ] `frontend-test.yml` (replaced by frontend.yml)
    - [ ] `contract-test.yml` (replaced by contract-specific tests)
    - [ ] `basic-ci.yml` (redundant)
  - [ ] Verify new workflows are working:
    - [ ] frontend.yml
    - [ ] nft-contract.yml
    - [ ] coloring-contract.yml
    - [ ] e2e-tests.yml

## 🤖 Discord Integration
- [ ] GitHub Actions Notifications
  - [ ] Setup Discord webhook
  - [ ] Create notification workflow
  - [ ] Add notifications for:
    - [ ] Workflow starts
    - [ ] Build status
    - [ ] Test results
    - [ ] Deployment status
    - [ ] E2E test reports
- [ ] Contract Event Monitor
  - [ ] Setup Discord bot
  - [ ] Implement event listeners for:
    - [ ] NFT mints
    - [ ] Color changes
    - [ ] Area transfers
    - [ ] Price updates
  - [ ] Add command interface for:
    - [ ] Contract statistics
    - [ ] Recent activities
    - [ ] Price checks

## 🔧 Environment Setup
- [ ] Documentation
  - [ ] Create comprehensive setup guide
  - [ ] Document all required environment variables
  - [ ] Add troubleshooting section
- [ ] Environment Files
  - [ ] Update .env.example files
  - [ ] Add validation scripts
  - [ ] Document external dependencies
- [ ] Required External Setup
  - [ ] List of required accounts:
    - [ ] Stargaze testnet/mainnet
    - [ ] Discord bot
    - [ ] GitHub secrets
  - [ ] Required tools and versions:
    - [ ] Node.js
    - [ ] Rust
    - [ ] Docker
    - [ ] CosmJS
  - [ ] Network access requirements:
    - [ ] RPC endpoints
    - [ ] API access
    - [ ] Webhook URLs
