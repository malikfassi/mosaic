#!/bin/bash

# Default values
NETWORK="testnet"
CANVAS_WIDTH=1000
CANVAS_HEIGHT=1000
PIXEL_PRICE=1000000
COLOR_PRICE=500000
COOLDOWN=3600

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --network)
      NETWORK="$2"
      shift 2
      ;;
    --code-ids)
      CODE_IDS_FILE="$2"
      shift 2
      ;;
    --canvas-width)
      CANVAS_WIDTH="$2"
      shift 2
      ;;
    --canvas-height)
      CANVAS_HEIGHT="$2"
      shift 2
      ;;
    --pixel-price)
      PIXEL_PRICE="$2"
      shift 2
      ;;
    --color-price)
      COLOR_PRICE="$2"
      shift 2
      ;;
    --cooldown)
      COOLDOWN="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1"
      exit 1
      ;;
  esac
done

# Validate required arguments
if [ -z "$CODE_IDS_FILE" ]; then
  echo "Error: --code-ids argument is required"
  exit 1
fi

# Load code IDs from file
if [ ! -f "$CODE_IDS_FILE" ]; then
  echo "Error: Code IDs file not found: $CODE_IDS_FILE"
  exit 1
fi

# Parse code IDs
declare -A CODE_IDS
while IFS=': ' read -r contract id; do
  CODE_IDS[$contract]=$id
done < "$CODE_IDS_FILE"

# Set network-specific variables
if [ "$NETWORK" = "testnet" ]; then
  CHAIN_ID=${STARGAZE_TESTNET_CHAIN_ID:-"elgafar-1"}
  NODE=${STARGAZE_TESTNET_RPC:-"https://rpc.elgafar-1.stargaze-apis.com:443"}
else
  CHAIN_ID=${STARGAZE_CHAIN_ID:-"stargaze-1"}
  NODE=${STARGAZE_RPC:-"https://rpc.stargaze-apis.com:443"}
fi

# Common transaction flags
TX_FLAGS="--gas auto --gas-adjustment 1.3 -y --output json --chain-id $CHAIN_ID --node $NODE"

# Function to extract contract address from instantiation response
get_contract_address() {
  local response=$1
  echo "$response" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value'
}

echo "Instantiating contracts on $NETWORK..."
echo "Chain ID: $CHAIN_ID"
echo "Node: $NODE"

# Instantiate factory contract
echo "Instantiating factory contract..."
FACTORY_INIT="{\"name\":\"Pixel NFTs\",\"symbol\":\"PIXEL\",\"canvas_width\":$CANVAS_WIDTH,\"canvas_height\":$CANVAS_HEIGHT,\"pixel_price\":\"$PIXEL_PRICE\",\"color_change_price\":\"$COLOR_PRICE\",\"color_change_cooldown\":$COOLDOWN,\"nft_code_id\":${CODE_IDS[sg721-pixel]},\"coloring_code_id\":${CODE_IDS[coloring]},\"collection_image\":\"ipfs://...\"}"

FACTORY_RESPONSE=$(starsd tx wasm instantiate ${CODE_IDS[factory]} "$FACTORY_INIT" \
  --label "Pixel Factory" \
  --admin $(starsd keys show -a deployer) \
  --from deployer \
  $TX_FLAGS)

FACTORY_ADDRESS=$(get_contract_address "$FACTORY_RESPONSE")
echo "Factory contract instantiated at: $FACTORY_ADDRESS"

# Save contract addresses
echo "Contract Addresses:" > contract_addresses.txt
echo "factory: $FACTORY_ADDRESS" >> contract_addresses.txt

# Query factory to get NFT and coloring contract addresses
sleep 5 # Wait for chain to process
CONTRACTS_QUERY='{"get_contracts":{}}'
CONTRACTS_RESPONSE=$(starsd query wasm contract-state smart $FACTORY_ADDRESS "$CONTRACTS_QUERY" --node $NODE -o json)

NFT_ADDRESS=$(echo "$CONTRACTS_RESPONSE" | jq -r '.[0]')
COLORING_ADDRESS=$(echo "$CONTRACTS_RESPONSE" | jq -r '.[1]')

echo "nft: $NFT_ADDRESS" >> contract_addresses.txt
echo "coloring: $COLORING_ADDRESS" >> contract_addresses.txt

echo "All contracts instantiated successfully!"
echo "Contract addresses saved to: contract_addresses.txt" 