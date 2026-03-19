use anchor_client::Program;
use anchor_lang::system_program;
use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use living_nft_engine::{NFTTraits, UpdateNFTTraits};
use reqwest;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info, warn};

#[derive(Parser)]
#[command(name = "living-nft-oracle")]
#[command(about = "Oracle service for updating dynamic NFTs based on real-world data")]
struct Cli {
    /// RPC endpoint URL
    #[arg(long, default_value = "http://127.0.0.1:8899")]
    rpc_url: String,

    /// Oracle wallet keypair file path
    #[arg(long, default_value = "~/.config/solana/id.json")]
    keypair_path: String,

    /// Program ID of the living NFT engine
    #[arg(long, default_value = "11111111111111111111111111111111")]
    program_id: String,

    /// Update interval in seconds
    #[arg(long, default_value = "300")]
    interval: u64,

    /// Weather API endpoint (mock)
    #[arg(long, default_value = "https://api.openweathermap.org/data/2.5/weather")]
    weather_api: String,

    /// API key for weather service
    #[arg(long)]
    api_key: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherData {
    weather: Vec<WeatherCondition>,
    main: MainWeather,
    wind: Wind,
    dt: i64,
    name: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct WeatherCondition {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct MainWeather {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: u32,
    humidity: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct Wind {
    speed: f64,
    deg: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct MockApiResponse {
    temperature: f64,
    humidity: u32,
    wind_speed: f64,
    weather_condition: String,
    timestamp: i64,
    location: String,
}

struct Oracle {
    rpc_client: RpcClient,
    program: Program,
    oracle_keypair: Keypair,
    program_id: Pubkey,
    config_pubkey: Pubkey,
}

impl Oracle {
    fn new(rpc_url: &str, keypair_path: &str, program_id: &str) -> Result<Self> {
        let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());
        
        // Load oracle keypair
        let oracle_keypair = if keypair_path.starts_with('~') {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            format!("{}{}", home, &keypair_path[1..])
        } else {
            keypair_path.to_string()
        };
        
        let oracle_keypair = Keypair::read_from_file(&oracle_keypair)
            .map_err(|e| anyhow::anyhow!("Failed to read keypair from {}: {}", oracle_keypair, e))?;

        let program_id = Pubkey::from_str(program_id)?;
        
        // Find config PDA
        let (config_pubkey, _) = Pubkey::find_program_address(&[b"config"], &program_id);

        // Initialize anchor client
        let program = Program {
            id: program_id,
            provider: anchor_client::Provider {
                wallet: anchor_client::Wallet::new(oracle_keypair.insecure_clone()),
                client: rpc_client.clone(),
                commitment: CommitmentConfig::confirmed(),
            },
        };

        Ok(Self {
            rpc_client,
            program,
            oracle_keypair,
            program_id,
            config_pubkey,
        })
    }

    async fn fetch_weather_data(&self, api_url: &str, api_key: Option<&str>) -> Result<MockApiResponse> {
        // For demo purposes, return mock data
        // In production, you would make actual API calls
        let mock_data = MockApiResponse {
            temperature: 25.5,
            humidity: 65,
            wind_speed: 10.2,
            weather_condition: "partly_cloudy".to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as i64,
            location: "San Francisco".to_string(),
        };

        // Uncomment below for real API call
        /*
        let client = reqwest::Client::new();
        let url = if let Some(key) = api_key {
            format!("{}?q=San Francisco&appid={}&units=metric", api_url, key)
        } else {
            format!("{}?q=San Francisco&units=metric", api_url)
        };

        let response = client.get(&url).send().await?;
        let weather_data: WeatherData = response.json().await?;

        let mock_data = MockApiResponse {
            temperature: weather_data.main.temp,
            humidity: weather_data.main.humidity,
            wind_speed: weather_data.wind.speed,
            weather_condition: weather_data.weather[0].main.to_lowercase(),
            timestamp: weather_data.dt,
            location: weather_data.name,
        };
        */

        Ok(mock_data)
    }

    fn calculate_traits(&self, weather_data: &MockApiResponse) -> NFTTraits {
        // Map weather data to NFT traits
        let background = match weather_data.weather_condition.as_str() {
            "clear" => 0,      // Sunny background
            "clouds" => 1,     // Cloudy background
            "rain" => 2,       // Rainy background
            "snow" => 3,       // Snowy background
            "thunderstorm" => 4, // Storm background
            _ => 5,            // Default background
        };

        let mood = if weather_data.temperature > 25.0 {
            200 // Happy mood when warm
        } else if weather_data.temperature < 10.0 {
            50  // Sad mood when cold
        } else {
            125 // Neutral mood
        };

        let activity = match weather_data.wind_speed {
            0.0..=5.0 => 100,   // Calm activity
            5.1..=15.0 => 150,  // Moderate activity
            _ => 200,            // High activity
        };

        let weather_effect = background * 40; // Scale weather effect

        let hour = (weather_data.timestamp / 3600) % 24;
        let time_of_day = match hour {
            6..=11 => 50,   // Morning
            12..=17 => 100, // Afternoon
            18..=21 => 150, // Evening
            _ => 200,        // Night
        };

        let special_event = if weather_data.humidity > 80 {
            180 // High humidity special effect
        } else {
            50  // Normal
        };

        let power_level = (weather_data.temperature * 10.0) as u16 + weather_data.wind_speed as u16;
        let rarity_score = (background as u16 * 1000) + (mood as u16) + (activity as u16);

        NFTTraits {
            background,
            mood,
            activity,
            weather_effect,
            time_of_day,
            special_event,
            power_level,
            rarity_score,
        }
    }

    async fn update_nft(&self, nft_pubkey: &Pubkey, new_traits: NFTTraits) -> Result<()> {
        info!("Updating NFT {} with new traits", nft_pubkey);

        // Find NFT PDA
        let (nft_pda, _) = Pubkey::find_program_address(&[b"nft", nft_pubkey.as_ref()], &self.program_id);

        // Create update instruction
        let update_accounts = living_nft_engine::accounts::UpdateNFTTraits {
            config: self.config_pubkey,
            nft: nft_pda,
            oracle: self.oracle_keypair.pubkey(),
        };

        let update_instruction = living_nft_engine::instruction::UpdateNFTTraits {
            new_traits,
            new_uri: None, // Could update URI based on new traits
        };

        let instruction = self.program
            .request()
            .accounts(update_accounts)
            .args(update_instruction)
            .instructions()?;

        // Create and send transaction
        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;
        let transaction = Transaction::new_signed_with_payer(
            &instruction,
            Some(&self.oracle_keypair.pubkey()),
            &[&self.oracle_keypair],
            recent_blockhash,
        );

        let signature = self.rpc_client.send_and_confirm_transaction(&transaction)?;
        info!("NFT update transaction confirmed: {}", signature);

        Ok(())
    }

    async fn run_update_cycle(&self, api_url: &str, api_key: Option<&str>) -> Result<()> {
        info!("Starting oracle update cycle");

        // Fetch weather data
        let weather_data = self.fetch_weather_data(api_url, api_key).await?;
        info!("Fetched weather data: {:?}", weather_data);

        // Calculate new traits
        let new_traits = self.calculate_traits(&weather_data);
        info!("Calculated new traits: {:?}", new_traits);

        // In a real implementation, you would:
        // 1. Query all NFTs minted by this program
        // 2. Update each NFT with the new traits
        // For demo purposes, we'll just log the calculation

        // Example of how to update a specific NFT:
        // let nft_pubkey = Pubkey::from_str("...")?;
        // self.update_nft(&nft_pubkey, new_traits).await?;

        info!("Oracle update cycle completed successfully");
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    info!("Starting Living NFT Oracle Service");
    info!("RPC URL: {}", cli.rpc_url);
    info!("Update interval: {} seconds", cli.interval);

    let oracle = Oracle::new(&cli.rpc_url, &cli.keypair_path, &cli.program_id)?;

    let mut interval_timer = interval(Duration::from_secs(cli.interval));

    loop {
        tokio::select! {
            _ = interval_timer.tick() => {
                if let Err(e) = oracle.run_update_cycle(&cli.weather_api, cli.api_key.as_deref()).await {
                    error!("Error during update cycle: {}", e);
                }
            }
        }
    }
}
