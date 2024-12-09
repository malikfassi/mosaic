# Mosaic Vending Minter Contract

This contract serves as a vending machine for minting Mosaic Tile NFTs. It provides functionality for both random and position-based minting, with support for batch operations.

## Features

- **Position-based Minting**: Mint tiles at specific positions
- **Random Minting**: Mint tiles at automatically selected positions
- **Batch Operations**: Mint multiple tiles in a single transaction
- **Payment Handling**: Secure payment processing in STARS tokens
- **Configurable Settings**: Adjustable pricing and batch limits

## Contract State

### Config
```rust
pub struct Config {
    pub mosaic_nft_address: Addr,    // The mosaic NFT contract address
    pub payment_address: Addr,        // The payment address where funds are sent
    pub unit_price: Uint128,         // The cost of minting one tile
    pub max_batch_size: u32,         // Maximum number of tiles in a batch mint
    pub random_minting_enabled: bool, // Whether random minting is enabled
    pub position_minting_enabled: bool, // Whether position-based minting is enabled
}
```

### Storage
- `CONFIG`: Stores contract configuration
- `POSITION_TOKENS`: Maps positions to token IDs
- `TOTAL_MINTED`: Tracks total number of minted tiles
- `NEXT_POSITION`: Tracks next available position for random minting

## Messages

### InstantiateMsg
Initialize the contract with configuration parameters:
```rust
pub struct InstantiateMsg {
    pub mosaic_nft_address: String,
    pub payment_address: String,
    pub unit_price: Uint128,
    pub max_batch_size: u32,
    pub random_minting_enabled: bool,
    pub position_minting_enabled: bool,
}
```

### ExecuteMsg
Available execution messages:
- `MintRandom`: Mint a tile at a random position
- `MintPosition`: Mint a tile at a specific position
- `BatchMintRandom`: Mint multiple tiles at random positions
- `BatchMintPositions`: Mint multiple tiles at specific positions
- `UpdateConfig`: Update contract configuration

### QueryMsg
Available query messages:
- `Config`: Get current contract configuration
- `MintPosition`: Check if a position is available
- `MintCount`: Get total number of minted tiles
- `MintPrice`: Calculate price for minting tiles
- `MintablePositions`: List available positions

## Usage Examples

### Mint a Single Tile
```rust
// Position-based minting
let msg = ExecuteMsg::MintPosition {
    position: Position { x: 5, y: 5 },
    color: Color { r: 255, g: 0, b: 0 },
};

// Random minting
let msg = ExecuteMsg::MintRandom {
    color: Color { r: 255, g: 0, b: 0 },
};
```

### Batch Mint Tiles
```rust
// Batch position-based minting
let msg = ExecuteMsg::BatchMintPositions {
    mints: vec![
        (Position { x: 1, y: 1 }, Color { r: 255, g: 0, b: 0 }),
        (Position { x: 2, y: 2 }, Color { r: 0, g: 255, b: 0 }),
    ],
};

// Batch random minting
let msg = ExecuteMsg::BatchMintRandom {
    count: 2,
    colors: vec![
        Color { r: 255, g: 0, b: 0 },
        Color { r: 0, g: 255, b: 0 },
    ],
};
```

## Integration

The contract must be set as the minter in the Mosaic NFT contract. This is done by calling `UpdateMinter` on the NFT contract:

```rust
let msg = NFTExecuteMsg::UpdateMinter {
    minter: Some(vending_contract_address),
};
```

## Error Handling

The contract handles various error cases:
- Invalid positions
- Insufficient payment
- Position already taken
- Batch size exceeded
- Color count mismatch
- Unauthorized operations
- Disabled features

## Testing

The contract includes both unit tests and integration tests:
- Unit tests cover individual function behavior
- Integration tests verify interaction with the NFT contract
- Payment handling tests ensure secure transactions
- Error propagation tests validate error handling

Run tests with:
```bash
cargo test
```

## Security Considerations

1. Payment Validation
   - Exact payment amount required
   - Only accepts STARS tokens
   - No refunds for failed transactions

2. Position Management
   - Validates position boundaries
   - Prevents double minting
   - Sequential position allocation for random minting

3. Access Control
   - Only owner can update configuration
   - Only authorized minter can mint NFTs
   - Public minting with correct payment

## License

Apache License 2.0 