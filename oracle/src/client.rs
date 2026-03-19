use crate::traits::NFTTraits;
use crate::weather::{WeatherClient, MockApiResponse};
use anchor_client::Program;
use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::Keypair,
    transaction::Transaction,
};
use std::str::FromStr;

pub struct OracleClient {
    pub rpc_client: RpcClient,
    pub program: Program,
    pub oracle_keypair: Keypair,
    pub program_id: Pubkey,
    pub config_pubkey: Pubkey,
    pub weather_client: WeatherClient,
}

impl OracleClient {
    pub fn new(
        rpc_url: &str,
        keypair_path: &str,
        program_id: &str,
        weather_api_url: String,
        weather_api_key: Option<String>,
    ) -> Result<Self> {
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

        let weather_client = WeatherClient::new(weather_api_url, weather_api_key);

        Ok(Self {
            rpc_client,
            program,
            oracle_keypair,
            program_id,
            config_pubkey,
            weather_client,
        })
    }

    pub async fn fetch_weather_data(&self, location: &str) -> Result<MockApiResponse> {
        self.weather_client.fetch_mock_weather(location).await
    }

    pub fn calculate_traits(&self, weather_data: &MockApiResponse) -> NFTTraits {
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
        let mut rarity_score = (background as u16 * 1000) + (mood as u16) + (activity as u16);

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

    pub async fn update_nft(&self, nft_mint: &Pubkey, new_traits: NFTTraits) -> Result<String> {
        // Find NFT PDA
        let (nft_pda, _) = Pubkey::find_program_address(&[b"nft", nft_mint.as_ref()], &self.program_id);

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
        Ok(signature)
    }

    pub async fn update_all_nfts(&self, location: &str) -> Result<Vec<String>> {
        // Fetch weather data
        let weather_data = self.fetch_weather_data(location).await?;
        
        // Calculate new traits
        let new_traits = self.calculate_traits(&weather_data);

        // In a real implementation, you would:
        // 1. Query all NFTs minted by this program
        // 2. Update each NFT with the new traits
        // For now, return empty vec as placeholder
        Ok(vec![])
    }
}
