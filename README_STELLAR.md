# Living NFT Engine - Stellar Network Version

A complete Web3 boilerplate for deploying Dynamic NFTs (dNFTs) that automatically evolve their visual traits and metadata based on real-world events. Built with Rust Oracles to fetch off-chain API data and update NFTs on the **Stellar Network** using Soroban smart contracts.

## 🌟 Why Stellar?

- **Low Fees**: Minimal transaction costs compared to other blockchains
- **Fast Finality**: 3-5 second confirmation times
- **Built-in DeFi**: Native AMM, payments, and asset support
- **Growing Ecosystem**: Rapidly expanding dApp ecosystem
- **Developer-Friendly**: Excellent tooling and documentation

## Project Structure

```
living-nft-engine/
├── contracts/
│   └── living-nft-engine/     # Soroban smart contract
├── oracle/                    # Off-chain Rust Oracle service
├── backend/                   # API server for frontend
├── frontend/                  # React web application
├── tests/                     # Integration tests
├── scripts/                   # Deployment and utility scripts
└── soroban-project.toml       # Soroban project configuration
```

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Soroban CLI v21.0.0+
- Stellar CLI
- Node.js 18+
- PostgreSQL (optional for production)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/living-nft-engine.git
cd living-nft-engine

# Install Stellar tools
cargo install --locked soroban-cli
cargo install --locked stellar-cli

# Install frontend dependencies
cd frontend && npm install && cd ..
```

### 📦 Deploy Smart Contract

```bash
# Deploy to testnet
./scripts/deploy_stellar.sh

# Deploy to mainnet (when ready)
NETWORK=mainnet ./scripts/deploy_stellar.sh
```

### 🎮 Interact with Contract

```bash
# Interactive CLI for testing
./scripts/interact_stellar.sh testnet

# Or use specific commands:
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $YOUR_ADDRESS \
  --network testnet \
  -- function mint_nft \
  --arg $YOUR_ADDRESS \
  --arg "My Living NFT" \
  --arg "LIVING" \
  --arg "https://example.com/metadata.json" \
  --arg '{"background":128,"mood":128,"activity":128,"weather_effect":128,"time_of_day":128,"special_event":0,"power_level":1000,"rarity_score":1000}'
```

### 🌤️ Start Oracle Service

```bash
# Set environment variables
export CONTRACT_ID="your_contract_id"
export ORACLE_ADDRESS="your_oracle_address"

# Run oracle in continuous mode
cargo run --bin oracle_stellar -- --network testnet --mode continuous

# Or run single update
cargo run --bin oracle_stellar -- --network testnet --mode single
```

### 🖥️ Start Frontend

```bash
cd frontend
npm start
```

## 🏗️ Architecture

### Smart Contract (Soroban)

The Living NFT smart contract is built using Soroban, Stellar's smart contract platform:

```rust
#[contract]
pub struct LivingNFTContract;

#[contractimpl]
impl LivingNFTContract {
    pub fn initialize(env: Env, oracle_address: Address) { ... }
    pub fn mint_nft(env: Env, owner: Address, name: String, symbol: String, metadata_uri: String, initial_traits: NFTTraits) -> Bytes { ... }
    pub fn update_traits(env: Env, oracle: Address, token_id: Bytes, new_traits: NFTTraits) { ... }
    pub fn transfer(env: Env, from: Address, to: Address, token_id: Bytes) { ... }
}
```

**Key Features:**
- **Oracle Security**: Only authorized oracle addresses can update traits
- **Dynamic Traits**: 8 different trait categories that evolve based on real-world data
- **Token Standards**: Compatible with Stellar's token standards
- **Gas Optimized**: Efficient storage and computation patterns

### Oracle Service

The oracle service fetches real-world data and updates NFTs:

```rust
pub struct StellarTraitUpdater {
    client: StellarOracleClient,
    trait_calculator: TraitCalculator,
    update_interval: u64,
}
```

**Features:**
- **Weather Data**: Integration with weather APIs
- **Trait Calculation**: Intelligent mapping of real-world data to NFT traits
- **Batch Updates**: Efficient batch processing for multiple NFTs
- **Error Handling**: Robust error recovery and retry mechanisms

### Frontend dApp

React-based web application with Stellar wallet integration:

- **Wallet Connection**: Support for Stellar wallets (Albedo, Freighter, etc.)
- **NFT Gallery**: Browse and view all minted NFTs
- **Minting Interface**: Create new Living NFTs with custom traits
- **Real-time Updates**: Live trait visualization and updates
- **Oracle Dashboard**: Monitor oracle status and performance

## 🎯 NFT Traits

Living NFTs have 8 dynamic traits that evolve based on real-world data:

| Trait | Range | Influence |
|-------|-------|-----------|
| **Background** | 0-255 | Weather conditions (clear, cloudy, rain, snow, storm) |
| **Mood** | 0-255 | Temperature and atmospheric pressure |
| **Activity** | 0-255 | Wind speed and movement |
| **Weather Effect** | 0-255 | Current weather intensity |
| **Time of Day** | 0-255 | Local time (morning, afternoon, evening, night) |
| **Special Event** | 0-255 | Rare conditions (extreme weather, holidays) |
| **Power Level** | 0-65535 | Overall strength based on environmental factors |
| **Rarity Score** | 0-65535 | Calculated rarity based on trait combinations |

## 🔧 Configuration

### Environment Variables

```bash
# Stellar Configuration
CONTRACT_ID="your_contract_id"
ORACLE_ADDRESS="your_oracle_address"
NETWORK="testnet"  # testnet, mainnet, futurenet, standalone

# Oracle Configuration
UPDATE_INTERVAL=300  # seconds
WEATHER_API_KEY="your_weather_api_key"
WEATHER_LOCATION="New York"

# Backend Configuration
DATABASE_URL="postgresql://user:pass@localhost/living_nft"
BACKEND_PORT=3000

# Frontend Configuration
REACT_APP_STELLAR_NETWORK="testnet"
REACT_APP_CONTRACT_ID="your_contract_id"
```

### Soroban Project Configuration

```toml
# soroban-project.toml
[project]
name = "living-nft-engine"
version = "0.1.0"
description = "Living NFT Engine - Dynamic NFTs on Stellar Network"

[project.networks]
testnet = "Living NFT Engine - Testnet Network"
mainnet = "Living NFT Engine - Mainnet Network"
```

## 🧪 Testing

### Unit Tests

```bash
# Run contract tests
cd contracts/living-nft-engine
cargo test

# Run oracle tests
cd ../oracle
cargo test

# Run backend tests
cd ../backend
cargo test
```

### Integration Tests

```bash
# Deploy to testnet and run integration tests
./scripts/integration_test_stellar.sh
```

## 📊 Monitoring

### Oracle Status

Monitor oracle performance and status:

```bash
# Get oracle info
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $ORACLE_ADDRESS \
  --network testnet \
  -- function get_oracle_info \
  --arg $ORACLE_ADDRESS

# Get total supply
soroban contract invoke \
  --id $CONTRACT_ID \
  --source $ANY_ADDRESS \
  --network testnet \
  -- function get_total_supply
```

### Contract Events

Listen to contract events for real-time updates:

```bash
soroban contract events \
  --id $CONTRACT_ID \
  --network testnet \
  --start-ledger 100000 \
  --events "mint,update,transfer"
```

## 🔒 Security Considerations

### Oracle Security

- **Authorization**: Only pre-approved oracle addresses can update traits
- **Rate Limiting**: Configurable update intervals to prevent spam
- **Data Validation**: All external data is validated before processing
- **Error Handling**: Graceful failure modes to prevent contract disruption

### Contract Security

- **Access Control**: Strict ownership and permission models
- **Input Validation**: All inputs are validated and sanitized
- **Gas Limits**: Built-in gas limits to prevent DoS attacks
- **Upgrade Safety**: Contract upgrade patterns with safety checks

## 🚀 Deployment

### Testnet Deployment

```bash
# Deploy contract
./scripts/deploy_stellar.sh

# Start oracle
export NETWORK=testnet
cargo run --bin oracle_stellar -- --mode continuous

# Start backend
cd backend && cargo run

# Start frontend
cd frontend && npm start
```

### Mainnet Deployment

```bash
# Ensure you have sufficient XLM for deployment
stellar account balance

# Deploy with mainnet configuration
NETWORK=mainnet ./scripts/deploy_stellar.sh

# Update oracle for mainnet
export NETWORK=mainnet
export CONTRACT_ID="mainnet_contract_id"
export ORACLE_ADDRESS="mainnet_oracle_address"

cargo run --bin oracle_stellar -- --mode continuous
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Stellar Development Foundation** - For the amazing Soroban platform
- **Stellar Community** - For the excellent documentation and support
- **Living NFT Contributors** - For making dynamic NFTs a reality

## 📞 Support

- **Discord**: [Join our Discord](https://discord.gg/livingnft)
- **Twitter**: [@LivingNFT](https://twitter.com/livingnft)
- **Documentation**: [docs.livingnft.engine](https://docs.livingnft.engine)

---

**Built with ❤️ for the Stellar ecosystem**
