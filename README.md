# Living NFT Engine

A complete Web3 boilerplate for deploying Dynamic NFTs (dNFTs) that automatically evolve their visual traits and metadata based on real-world events. Built with Rust Oracles to fetch off-chain API data and update NFTs on Solana.

## Project Structure

```
living-nft-engine/
├── programs/
│   └── living-nft-engine/     # Anchor smart contract
├── oracle/                    # Off-chain Rust Oracle service
├── backend/                   # API server for frontend
├── frontend/                  # React web application
├── tests/                     # Integration tests
├── scripts/                   # Deployment and utility scripts
└── Anchor.toml               # Anchor configuration
```

### Components

1. **Smart Contract (Solana/Anchor)**: Handles NFT minting and trait updates with oracle-only access control
2. **Rust Oracle Daemon**: Off-chain service that fetches external data and updates NFTs
3. **Backend API**: REST API server for frontend-oracle communication
4. **Frontend dApp**: React application with Solana wallet integration and real-time visualization
3. **Dynamic Traits**: NFTs evolve based on weather, time, and other real-world factors

## Features

- ✅ Dynamic NFT traits that update based on real-world data
- ✅ Secure oracle-only update mechanism
- ✅ Weather-based trait calculation
- ✅ Time-of-day effects
- ✅ Rarity scoring system
- ✅ Event-driven updates
- ✅ Comprehensive logging and monitoring

## Prerequisites

- Rust 1.70+
- Solana CLI v1.18+
- Anchor CLI v0.30+
- Node.js 18+ (for Anchor)
- Git

## Installation

### 1. Clone and Setup

```bash
git clone <repository-url>
cd living-nft-engine
```

### 2. Install Solana and Anchor

```bash
# Install Solana CLI
sh -c "$(curl -sSfL https://release.solana.com/v1.18.0/install)"

# Install Anchor CLI
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install 0.30.1
avm use 0.30.1
```

### 3. Install Dependencies

```bash
# Install Rust dependencies
cargo build

# Install Node.js dependencies (for tests)
npm install
```

### 4. Environment Setup

```bash
# Copy environment template
cp .env.example .env

# Edit .env with your configuration
# Set your Solana keypair path and RPC URL
```

## Quick Start

### 1. Start Local Solana Validator

```bash
# Start local validator
solana-test-validator

# In another terminal, set cluster to localnet
solana config set --url localhost
```

### 2. Build and Deploy Smart Contract

```bash
# Build the program
anchor build

# Deploy to local validator
anchor deploy
```

### 3. Initialize the Program

```bash
# Run the initialization script
anchor run initialize
```

### 4. Start the Oracle Service

```bash
# Build the oracle
cd oracle
cargo build --release

# Run the oracle
./target/release/oracle \
  --rpc-url http://127.0.0.1:8899 \
  --keypair-path ~/.config/solana/id.json \
  --interval 60
```

## Smart Contract Details

### Core Instructions

#### `initialize(oracle_authority: Pubkey)`
Initializes the program configuration with the authorized oracle address.

#### `mint_nft(name, symbol, uri, initial_traits)`
Mints a new dynamic NFT with initial traits.

#### `update_nft_traits(new_traits, new_uri)`
Updates NFT traits (oracle-only access).

### NFT Traits Structure

```rust
pub struct NFTTraits {
    pub background: u8,      // Weather-based background
    pub mood: u8,           // Temperature-based mood
    pub activity: u8,       // Wind-speed based activity
    pub weather_effect: u8,  // Direct weather effect
    pub time_of_day: u8,    // Time-based visual changes
    pub special_event: u8,  // Special conditions
    pub power_level: u16,   // Combined power metric
    pub rarity_score: u16,  // Calculated rarity
}
```

## Oracle Service

### Configuration

The oracle service can be configured via command-line arguments or environment variables:

```bash
./oracle --help
```

### Weather Data Sources

Currently supports:
- **Mock Data**: Generates consistent mock weather data for testing
- **OpenWeatherMap API**: Real weather data (requires API key)

### Trait Calculation Logic

The oracle maps weather data to NFT traits:

- **Temperature** → Mood (cold=sad, warm=happy)
- **Wind Speed** → Activity level
- **Weather Condition** → Background scene
- **Time of Day** → Lighting effects
- **Humidity** → Special events

## Development

### Building

```bash
# Build all components
cargo build

# Build release version
cargo build --release

# Build only smart contract
anchor build

# Build only oracle
cd oracle && cargo build
```

### Testing

```bash
# Run Anchor tests
anchor test

# Run Rust unit tests
cargo test

# Run oracle tests
cd oracle && cargo test
```

### Local Development

```bash
# Start local validator with ledger reset
solana-test-validator --reset

# Deploy and test in watch mode
anchor test --skip-local-validator
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SOLANA_RPC_URL` | Solana RPC endpoint | `http://127.0.0.1:8899` |
| `SOLANA_KEYPAIR_PATH` | Oracle keypair file | `~/.config/solana/id.json` |
| `PROGRAM_ID` | Program ID | `11111111111111111111111111111111` |
| `ORACLE_UPDATE_INTERVAL` | Update frequency (seconds) | `300` |
| `WEATHER_API_KEY` | Weather API key | None |
| `DEFAULT_LOCATION` | Default location for weather | `San Francisco` |

### Anchor Configuration

Edit `Anchor.toml` for different deployment targets:

```toml
[programs.mainnet]
living_nft_engine = "YOUR_PROGRAM_ID"

[provider]
cluster = "mainnet-beta"
wallet = "path/to/your/keypair.json"
```

## Deployment

### Localnet

```bash
# Start validator
solana-test-validator

# Deploy
anchor deploy --provider.cluster localnet
```

### Devnet

```bash
# Switch to devnet
solana config set --url devnet

# Request SOL if needed
solana airdrop 2

# Deploy
anchor deploy --provider.cluster devnet
```

### Mainnet-Beta

```bash
# Switch to mainnet
solana config set --url mainnet-beta

# Deploy (requires sufficient SOL)
anchor deploy --provider.cluster mainnet-beta
```

## Security Considerations

1. **Oracle Authority**: Only the authorized oracle can update NFT traits
2. **Key Management**: Secure oracle keypair storage
3. **API Security**: Validate external API responses
4. **Rate Limiting**: Implement rate limiting for API calls
5. **Access Control**: Verify oracle signatures in smart contract

## Monitoring and Logging

The oracle service includes comprehensive logging:

```bash
# Set log level
RUST_LOG=debug ./oracle

# View logs
tail -f oracle.log
```

## Troubleshooting

### Common Issues

1. **Program ID Mismatch**: Update `declare_id!` in lib.rs with deployed program ID
2. **Oracle Authorization**: Ensure oracle keypair matches `oracle_authority` in config
3. **RPC Connection**: Verify RPC URL is accessible
4. **Insufficient SOL**: Fund oracle wallet for transaction fees

### Debug Commands

```bash
# Check program account
solana account <PROGRAM_ID>

# Check config account
solana account <CONFIG_PUBKEY>

# Check oracle balance
solana balance <ORACLE_PUBKEY>
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

MIT License - see LICENSE file for details.

## Support

- 📖 [Documentation](https://docs.solana.com/)
- 💬 [Discord Community](https://discord.gg/solana)
- 🐛 [Issue Tracker](https://github.com/your-repo/issues)

---

**Built with ❤️ using Solana and Anchor**
