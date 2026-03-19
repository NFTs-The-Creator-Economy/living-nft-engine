#!/bin/bash

# Stellar Soroban deployment script for Living NFT Engine

set -e

# Configuration
NETWORK="testnet"  # Change to "mainnet" for production
CONTRACT_DIR="contracts/living-nft-engine"
CONTRACT_WASM="$CONTRACT_DIR/target/wasm32-unknown-unknown/release/living_nft_engine.wasm"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Deploying Living NFT Engine to Stellar Network${NC}"
echo -e "${YELLOW}Network: $NETWORK${NC}"

# Check if soroban CLI is installed
if ! command -v soroban &> /dev/null; then
    echo -e "${RED}❌ Soroban CLI not found. Please install it first.${NC}"
    echo "Visit: https://developers.stellar.org/docs/soroban/getting-started/setup"
    exit 1
fi

# Build the contract
echo -e "${YELLOW}🔨 Building contract...${NC}"
cd "$CONTRACT_DIR"
cargo build --target wasm32-unknown-unknown --release

if [ ! -f "$CONTRACT_WASM" ]; then
    echo -e "${RED}❌ Contract build failed${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Contract built successfully${NC}"

# Deploy contract
echo -e "${YELLOW}📦 Deploying contract...${NC}"
cd ../..

# Get or create a deployer account
DEPLOYER_ADDRESS=$(soroban keys address --global)
echo -e "${YELLOW}Deployer address: $DEPLOYER_ADDRESS${NC}"

# Check if account has enough balance
BALANCE=$(soroban account balance --id $DEPLOYER_ADDRESS --network $NETWORK --asset native)
echo -e "${YELLOW}Current balance: $BALANCE XLM${NC}"

# Deploy the contract
CONTRACT_ID=$(soroban contract deploy \
    --wasm "$CONTRACT_WASM" \
    --source $DEPLOYER_ADDRESS \
    --network $NETWORK)

echo -e "${GREEN}✅ Contract deployed!${NC}"
echo -e "${GREEN}Contract ID: $CONTRACT_ID${NC}"

# Initialize the contract with oracle
echo -e "${YELLOW}🔧 Initializing contract...${NC}"
ORACLE_ADDRESS=$(soroban keys generate --global oracle)
echo -e "${YELLOW}Oracle address: $ORACLE_ADDRESS${NC}"

# Initialize contract
soroban contract invoke \
    --id $CONTRACT_ID \
    --source $DEPLOYER_ADDRESS \
    --network $NETWORK \
    -- function initialize \
    --arg $ORACLE_ADDRESS

echo -e "${GREEN}✅ Contract initialized with oracle${NC}"

# Save deployment info
DEPLOYMENT_FILE="deployment_$NETWORK.json"
cat > "$DEPLOYMENT_FILE" << EOF
{
  "network": "$NETWORK",
  "contract_id": "$CONTRACT_ID",
  "deployer_address": "$DEPLOYER_ADDRESS",
  "oracle_address": "$ORACLE_ADDRESS",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
}
EOF

echo -e "${GREEN}📄 Deployment info saved to $DEPLOYMENT_FILE${NC}"

# Test the deployment
echo -e "${YELLOW}🧪 Testing deployment...${NC}"
TOTAL_SUPPLY=$(soroban contract invoke \
    --id $CONTRACT_ID \
    --source $DEPLOYER_ADDRESS \
    --network $NETWORK \
    -- function get_total_supply)

echo -e "${GREEN}✅ Total supply: $TOTAL_SUPPLY${NC}"

echo -e "${GREEN}🎉 Deployment completed successfully!${NC}"
echo -e "${YELLOW}Contract ID: $CONTRACT_ID${NC}"
echo -e "${YELLOW}Oracle Address: $ORACLE_ADDRESS${NC}"
echo -e "${YELLOW}Deployer Address: $DEPLOYER_ADDRESS${NC}"
