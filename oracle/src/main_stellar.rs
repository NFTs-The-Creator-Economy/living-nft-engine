use clap::{Arg, Command};
use dotenv::dotenv;
use std::env;
use tracing::{info, error, warn};
use tracing_subscriber;

mod stellar_client;
mod traits;
mod weather;
mod client;

use stellar_client::{StellarOracleClient, StellarTraitUpdater};
use client::OracleClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let matches = Command::new("Living NFT Oracle")
        .version("0.1.0")
        .about("Oracle service for updating dynamic NFTs on Stellar Network")
        .arg(
            Arg::new("network")
                .long("network")
                .short('n')
                .value_name("NETWORK")
                .help("Stellar network (testnet/mainnet)")
                .default_value("testnet"),
        )
        .arg(
            Arg::new("contract-id")
                .long("contract-id")
                .short('c')
                .value_name("CONTRACT_ID")
                .help("Soroban contract ID"),
        )
        .arg(
            Arg::new("oracle-address")
                .long("oracle-address")
                .short('o')
                .value_name("ORACLE_ADDRESS")
                .help("Oracle Stellar address"),
        )
        .arg(
            Arg::new("rpc-url")
                .long("rpc-url")
                .short('r')
                .value_name("RPC_URL")
                .help("Stellar RPC URL"),
        )
        .arg(
            Arg::new("update-interval")
                .long("update-interval")
                .short('i')
                .value_name("SECONDS")
                .help("Update interval in seconds")
                .default_value("300"),
        )
        .arg(
            Arg::new("mode")
                .long("mode")
                .short('m')
                .value_name("MODE")
                .help("Operation mode (continuous/single)")
                .default_value("continuous"),
        )
        .get_matches();

    let network = matches.get_one::<String>("network").unwrap();
    let contract_id = matches.get_one::<String>("contract-id")
        .or_else(|| env::var("CONTRACT_ID").ok().as_ref())
        .expect("Contract ID is required");
    
    let oracle_address = matches.get_one::<String>("oracle-address")
        .or_else(|| env::var("ORACLE_ADDRESS").ok().as_ref())
        .expect("Oracle address is required");

    let rpc_url = get_rpc_url(network, matches.get_one::<String>("rpc-url"));
    let update_interval: u64 = matches.get_one::<String>("update-interval")
        .unwrap()
        .parse()
        .map_err(|_| anyhow::anyhow!("Invalid update interval"))?;

    let mode = matches.get_one::<String>("mode").unwrap();

    info!("Starting Living NFT Oracle for Stellar Network");
    info!("Network: {}", network);
    info!("Contract ID: {}", contract_id);
    info!("Oracle Address: {}", oracle_address);
    info!("RPC URL: {}", rpc_url);
    info!("Update Interval: {} seconds", update_interval);
    info!("Mode: {}", mode);

    // Initialize Stellar client
    let stellar_client = StellarOracleClient::new(
        &rpc_url,
        contract_id,
        oracle_address,
        get_network_passphrase(network),
    )?;

    // Create trait updater
    let updater = StellarTraitUpdater::new(stellar_client, update_interval);

    match mode.as_str() {
        "continuous" => {
            info!("Starting continuous updates...");
            updater.run_continuous_updates().await?;
        }
        "single" => {
            info!("Running single update...");
            let updated_count = updater.update_all_nfts().await?;
            info!("Updated {} NFTs", updated_count);
        }
        _ => {
            return Err(anyhow::anyhow!("Invalid mode. Use 'continuous' or 'single'"));
        }
    }

    Ok(())
}

fn get_rpc_url(network: &str, custom_url: Option<&String>) -> String {
    if let Some(url) = custom_url {
        return url.clone();
    }

    match network {
        "testnet" => "https://soroban-testnet.stellar.org".to_string(),
        "mainnet" => "https://soroban.stellar.org".to_string(),
        "futurenet" => "https://horizon-futurenet.stellar.org".to_string(),
        "standalone" => "http://localhost:8000".to_string(),
        _ => "https://soroban-testnet.stellar.org".to_string(),
    }
}

fn get_network_passphrase(network: &str) -> &str {
    match network {
        "testnet" => "Test SDF Network ; September 2015",
        "mainnet" => "Public Global Stellar Network ; September 2015",
        "futurenet" => "Test SDF Future Network ; October 2022",
        "standalone" => "Standalone Network ; February 2017",
        _ => "Test SDF Network ; September 2015",
    }
}
