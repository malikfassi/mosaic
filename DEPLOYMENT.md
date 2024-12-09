# Deployment Guide

This guide explains how to deploy the Pixel NFT contracts to Stargaze testnet or mainnet.

## Prerequisites

1. GitHub repository secrets:
   ```
   STARGAZE_TESTNET_RPC=https://rpc.elgafar-1.stargaze-apis.com:443
   STARGAZE_TESTNET_CHAIN_ID=elgafar-1
   STARGAZE_MAINNET_RPC=https://rpc.stargaze-apis.com:443
   STARGAZE_MAINNET_CHAIN_ID=stargaze-1
   DEPLOYMENT_WALLET_MNEMONIC=your_mnemonic_here
   CODECOV_TOKEN=your_codecov_token
   ```

2. Local environment for instantiation:
   ```bash
   # Install starsd CLI
   curl -s https://get.starsd.com | bash
   sudo mv starsd /usr/local/bin/

   # Set up environment variables
   export STARGAZE_TESTNET_RPC=https://rpc.elgafar-1.stargaze-apis.com:443
   export STARGAZE_TESTNET_CHAIN_ID=elgafar-1
   # Or for mainnet:
   # export STARGAZE_RPC=https://rpc.stargaze-apis.com:443
   # export STARGAZE_CHAIN_ID=stargaze-1

   # Import deployer wallet
   starsd keys add deployer --recover
   # Enter your mnemonic when prompted
   ```

## Deployment Process

### 1. Deploy Contract Code

1. Go to GitHub Actions
2. Select "Deploy Contracts" workflow
3. Click "Run workflow"
4. Choose network (testnet/mainnet)
5. Choose contracts to deploy (default: all)
6. Click "Run workflow"

The workflow will:
- Build and optimize the contracts
- Upload the contract code to the chain
- Save the code IDs as artifacts

### 2. Instantiate Contracts

After deployment, download the code IDs artifact and run the instantiation script:

```bash
# Download and extract code IDs
unzip deployment-info.zip
cd scripts

# For testnet with default values
./instantiate.sh --network testnet --code-ids ../code_ids.txt

# For testnet with custom values
./instantiate.sh \
  --network testnet \
  --code-ids ../code_ids.txt \
  --canvas-width 1000 \
  --canvas-height 1000 \
  --pixel-price 1000000 \
  --color-price 500000 \
  --cooldown 3600

# For mainnet (be extra careful!)
./instantiate.sh --network mainnet --code-ids ../code_ids.txt
```

The script will:
- Instantiate the factory contract
- Factory will instantiate NFT and coloring contracts
- Save all contract addresses to `contract_addresses.txt`

## Cost Optimization

To minimize deployment costs:

1. Test thoroughly on testnet first
2. Deploy contracts one at a time if needed:
   ```bash
   # Example: Deploy only factory first
   gh workflow run deploy.yml -f network=testnet -f contracts=factory
   ```

3. Use appropriate gas settings:
   - The scripts use `--gas auto` with 1.3 adjustment
   - Monitor actual gas usage on testnet
   - Adjust if needed in `deploy.yml` and `instantiate.sh`

4. Batch operations when possible:
   - The factory contract handles NFT and coloring contract instantiation
   - This saves on transaction costs

## Verification

After deployment:

1. Verify contract addresses:
   ```bash
   # Query factory contract
   starsd query wasm contract-state smart $FACTORY_ADDRESS '{"get_contracts":{}}'
   ```

2. Test basic functionality:
   ```bash
   # Example: Mint a pixel
   starsd tx wasm execute $FACTORY_ADDRESS '{"mint_pixel":{"x":0,"y":0,"owner":"stars..."}}' --amount 1000000ustars
   ```

## Troubleshooting

1. If deployment fails:
   - Check GitHub Actions logs
   - Verify wallet has sufficient funds
   - Confirm RPC endpoints are responsive

2. If instantiation fails:
   - Check `contract_addresses.txt` for partial success
   - Review chain error messages
   - Verify code IDs match deployment

3. Common issues:
   - Insufficient gas: Increase gas adjustment
   - RPC timeout: Try alternative endpoints
   - Invalid admin: Verify deployer address 