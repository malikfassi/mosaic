# Mosaic Configuration

This document describes the configuration parameters for the Mosaic NFT project.

## Configuration File

The project configuration is stored in `config.json` at the root of the repository.

### Tile Configuration

```json
"tile": {
    "size": 32,        // Size of each tile (32x32 pixels)
    "max_count": 1024  // Maximum number of tiles (32x32 grid)
}
```

- `size`: The size of each tile in pixels. Default is 32, creating a 32x32 pixel grid per tile.
- `max_count`: The maximum number of tiles that can be minted. Default is 1024, creating a 32x32 grid of tiles.

### Fee Configuration

```json
"fees": {
    "base_fee": {
        "denom": "ustars",
        "amount": "20000000"    // 20 STARS
    },
    "mint_price": {
        "denom": "ustars",
        "amount": "100000000"  // 100 STARS
    },
    "developer_fee_percent": 10
}
```

- `base_fee`: The base fee for updating a pixel for the base duration
  - `denom`: The denomination of the fee (ustars)
  - `amount`: The amount in the smallest unit (20 STARS = 20,000,000 ustars)
- `mint_price`: The price to mint a new tile
  - `denom`: The denomination of the mint price (ustars)
  - `amount`: The amount in the smallest unit (100 STARS = 100,000,000 ustars)
- `developer_fee_percent`: The percentage of fees that go to the developer (10%)

## Fee Scaling

The fee scales quadratically with duration using the formula: `fee = base_fee * (duration/24h)²`

This creates a smooth, continuous curve where:

| Duration | Relative Duration | Multiplier | Total Fee |
|----------|------------------|------------|-----------|
| 1 second | 0.001%          | 0.0001x    | 0.002 STARS |
| 1 hour   | 4.17%           | 0.0017x    | 0.034 STARS |
| 12 hours | 50%             | 0.25x      | 5 STARS     |
| 24 hours | 100%            | 1x         | 20 STARS    |
| 36 hours | 150%            | 2.25x      | 45 STARS    |
| 48 hours | 200%            | 4x         | 80 STARS    |
| 72 hours | 300%            | 9x         | 180 STARS   |
| And so on...               |            |              |

This continuous quadratic scaling provides several benefits:
- Precise to the second, allowing for very fine-grained duration control
- Very short durations (seconds to minutes) are extremely cheap
- Medium durations (hours) are affordable
- Long durations become increasingly expensive in a smooth, predictable way
- No sudden fee jumps at period boundaries
- Users can optimize their timing to get exactly the duration they need

Example use cases and fees:
1. Quick update (1 second): 0.002 STARS
2. Short message (1 hour): 0.034 STARS
3. Half-day display (12 hours): 5 STARS
4. Full-day display (24 hours): 20 STARS
5. Extended display (36 hours): 45 STARS
6. Long-term display (48 hours): 80 STARS
7. Multi-day display (72 hours): 180 STARS

The total fee is distributed between:
- Developer: 10% of total fee
- Tile Owner: 90% of total fee