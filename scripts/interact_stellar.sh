#!/bin/bash

# Stellar Soroban interaction script for Living NFT Engine

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Load deployment info
NETWORK=${1:-"testnet"}
DEPLOYMENT_FILE="deployment_$NETWORK.json"

if [ ! -f "$DEPLOYMENT_FILE" ]; then
    echo -e "${RED}❌ Deployment file not found: $DEPLOYMENT_FILE${NC}"
    echo "Please run deploy_stellar.sh first"
    exit 1
fi

CONTRACT_ID=$(jq -r '.contract_id' "$DEPLOYMENT_FILE")
ORACLE_ADDRESS=$(jq -r '.oracle_address' "$DEPLOYMENT_FILE")
DEPLOYER_ADDRESS=$(jq -r '.deployer_address' "$DEPLOYMENT_FILE")

echo -e "${GREEN}🎮 Living NFT Engine - Interaction Script${NC}"
echo -e "${YELLOW}Network: $NETWORK${NC}"
echo -e "${YELLOW}Contract ID: $CONTRACT_ID${NC}"
echo -e "${YELLOW}Oracle Address: $ORACLE_ADDRESS${NC}"

# Function to display menu
show_menu() {
    echo -e "\n${BLUE}📋 Available Operations:${NC}"
    echo "1. Mint a new NFT"
    echo "2. Get NFT by token ID"
    echo "3. Get owner's NFTs"
    echo "4. Update NFT traits (Oracle only)"
    echo "5. Transfer NFT"
    echo "6. Get total supply"
    echo "7. Get oracle info"
    echo "8. Authorize new oracle"
    echo "9. Exit"
    echo -n -e "${YELLOW}Choose an option (1-9): ${NC}"
}

# Function to mint NFT
mint_nft() {
    echo -e "${YELLOW}🪙 Minting new NFT...${NC}"
    
    echo -n "Enter owner address (leave empty to use deployer): "
    read OWNER_INPUT
    OWNER=${OWNER_INPUT:-$DEPLOYER_ADDRESS}
    
    echo -n "Enter NFT name: "
    read NAME
    
    echo -n "Enter NFT symbol: "
    read SYMBOL
    
    echo -n "Enter metadata URI: "
    read METADATA_URI
    
    echo -e "${YELLOW}Using default traits (you can update them later)...${NC}"
    
    # Mint the NFT
    TOKEN_ID=$(soroban contract invoke \
        --id $CONTRACT_ID \
        --source $OWNER \
        --network $NETWORK \
        -- function mint_nft \
        --arg $OWNER \
        --arg "$NAME" \
        --arg "$SYMBOL" \
        --arg "$METADATA_URI" \
        --arg '{
            "background": 128,
            "mood": 128,
            "activity": 128,
            "weather_effect": 128,
            "time_of_day": 128,
            "special_event": 0,
            "power_level": 1000,
            "rarity_score": 1000
        }' | jq -r '.result')
    
    echo -e "${GREEN}✅ NFT minted successfully!${NC}"
    echo -e "${GREEN}Token ID: $TOKEN_ID${NC}"
}

# Function to get NFT
get_nft() {
    echo -e "${YELLOW}🔍 Getting NFT...${NC}"
    echo -n "Enter token ID: "
    read TOKEN_ID
    
    NFT=$(soroban contract invoke \
        --id $CONTRACT_ID \
        --source $DEPLOYER_ADDRESS \
        --network $NETWORK \
        -- function get_nft \
        --arg $TOKEN_ID)
    
    echo -e "${GREEN}📄 NFT Details:${NC}"
    echo "$NFT" | jq '.'
}

# Function to get owner's NFTs
get_owner_nfts() {
    echo -e "${YELLOW}👤 Getting owner's NFTs...${NC}"
    echo -n "Enter owner address (leave empty to use deployer): "
    read OWNER_INPUT
    OWNER=${OWNER_INPUT:-$DEPLOYER_ADDRESS}
    
    NFTS=$(soroban contract invoke \
        --id $CONTRACT_ID \
        --source $DEPLOYER_ADDRESS \
        --network $NETWORK \
        -- function get_owner_nfts \
        --arg $OWNER)
    
    echo -e "${GREEN}📄 Owner's NFTs:${NC}"
    echo "$NFTS" | jq '.'
}

# Function to update traits
update_traits() {
    echo -e "${YELLOW}🔄 Updating NFT traits...${NC}"
    echo -n "Enter token ID: "
    read TOKEN_ID
    
    echo -e "${YELLOW}Enter new traits (JSON format):${NC}"
    echo "Example: {\"background\": 255, \"mood\": 200, \"activity\": 180, \"weather_effect\": 150, \"time_of_day\": 120, \"special_event\": 50, \"power_level\": 1500, \"rarity_score\": 2000}"
    echo -n "Traits: "
    read TRAITS
    
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source $ORACLE_ADDRESS \
        --network $NETWORK \
        -- function update_traits \
        --arg $ORACLE_ADDRESS \
        --arg $TOKEN_ID \
        --arg "$TRAITS"
    
    echo -e "${GREEN}✅ NFT traits updated successfully!${NC}"
}

# Function to transfer NFT
transfer_nft() {
    echo -e "${YELLOW}📤 Transferring NFT...${NC}"
    echo -n "Enter from address: "
    read FROM
    
    echo -n "Enter to address: "
    read TO
    
    echo -n "Enter token ID: "
    read TOKEN_ID
    
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source $FROM \
        --network $NETWORK \
        -- function transfer \
        --arg $FROM \
        --arg $TO \
        --arg $TOKEN_ID
    
    echo -e "${GREEN}✅ NFT transferred successfully!${NC}"
}

# Function to get total supply
get_total_supply() {
    echo -e "${YELLOW}📊 Getting total supply...${NC}"
    
    TOTAL_SUPPLY=$(soroban contract invoke \
        --id $CONTRACT_ID \
        --source $DEPLOYER_ADDRESS \
        --network $NETWORK \
        -- function get_total_supply | jq -r '.result')
    
    echo -e "${GREEN}📈 Total Supply: $TOTAL_SUPPLY${NC}"
}

# Function to get oracle info
get_oracle_info() {
    echo -e "${YELLOW}🔮 Getting oracle info...${NC}"
    echo -n "Enter oracle address (leave empty to use default): "
    read ORACLE_INPUT
    ORACLE=${ORACLE_INPUT:-$ORACLE_ADDRESS}
    
    ORACLE_DATA=$(soroban contract invoke \
        --id $CONTRACT_ID \
        --source $DEPLOYER_ADDRESS \
        --network $NETWORK \
        -- function get_oracle_info \
        --arg $ORACLE)
    
    echo -e "${GREEN}📄 Oracle Information:${NC}"
    echo "$ORACLE_DATA" | jq '.'
}

# Function to authorize oracle
authorize_oracle() {
    echo -e "${YELLOW}👑 Authorizing new oracle...${NC}"
    echo -n "Enter new oracle address: "
    read NEW_ORACLE
    
    soroban contract invoke \
        --id $CONTRACT_ID \
        --source $DEPLOYER_ADDRESS \
        --network $NETWORK \
        -- function authorize_oracle \
        --arg $DEPLOYER_ADDRESS \
        --arg $NEW_ORACLE
    
    echo -e "${GREEN}✅ Oracle authorized successfully!${NC}"
}

# Main loop
while true; do
    show_menu
    read choice
    echo
    
    case $choice in
        1) mint_nft ;;
        2) get_nft ;;
        3) get_owner_nfts ;;
        4) update_traits ;;
        5) transfer_nft ;;
        6) get_total_supply ;;
        7) get_oracle_info ;;
        8) authorize_oracle ;;
        9) 
            echo -e "${GREEN}👋 Goodbye!${NC}"
            exit 0 
            ;;
        *) 
            echo -e "${RED}❌ Invalid option. Please try again.${NC}" 
            ;;
    esac
    
    echo -e "\n${BLUE}Press Enter to continue...${NC}"
    read
done
