use soroban_sdk::{Address, Bytes, Env, String};
use crate::{LivingNFTContract, LivingNFT, NFTTraits, DataKey};

#[test]
fn test_initialize() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LivingNFTContract);
    let client = LivingNFTContractClient::new(&env, &contract_id);
    
    let oracle = Address::generate(&env);
    
    client.initialize(&oracle);
    
    let oracle_info = client.get_oracle_info(&oracle);
    assert!(oracle_info.is_some());
    assert!(oracle_info.unwrap().is_authorized);
}

#[test]
fn test_mint_nft() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LivingNFTContract);
    let client = LivingNFTContractClient::new(&env, &contract_id);
    
    let oracle = Address::generate(&env);
    let owner = Address::generate(&env);
    
    client.initialize(&oracle);
    
    let initial_traits = NFTTraits {
        background: 128,
        mood: 128,
        activity: 128,
        weather_effect: 128,
        time_of_day: 128,
        special_event: 0,
        power_level: 1000,
        rarity_score: 1000,
    };
    
    let token_id = client.mint_nft(
        &owner,
        &String::from_str(&env, "Test NFT"),
        &String::from_str(&env, "TEST"),
        &String::from_str(&env, "https://example.com/metadata.json"),
        &initial_traits,
    );
    
    let nft = client.get_nft(&token_id).unwrap();
    assert_eq!(nft.owner, owner);
    assert_eq!(nft.name, String::from_str(&env, "Test NFT"));
    assert_eq!(nft.symbol, String::from_str(&env, "TEST"));
    assert_eq!(client.get_total_supply(), 1);
}

#[test]
fn test_update_traits() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LivingNFTContract);
    let client = LivingNFTContractClient::new(&env, &contract_id);
    
    let oracle = Address::generate(&env);
    let owner = Address::generate(&env);
    
    client.initialize(&oracle);
    
    let initial_traits = NFTTraits {
        background: 128,
        mood: 128,
        activity: 128,
        weather_effect: 128,
        time_of_day: 128,
        special_event: 0,
        power_level: 1000,
        rarity_score: 1000,
    };
    
    let token_id = client.mint_nft(
        &owner,
        &String::from_str(&env, "Test NFT"),
        &String::from_str(&env, "TEST"),
        &String::from_str(&env, "https://example.com/metadata.json"),
        &initial_traits,
    );
    
    let updated_traits = NFTTraits {
        background: 255,
        mood: 200,
        activity: 180,
        weather_effect: 150,
        time_of_day: 120,
        special_event: 50,
        power_level: 1500,
        rarity_score: 2000,
    };
    
    client.update_traits(&oracle, &token_id, &updated_traits);
    
    let nft = client.get_nft(&token_id).unwrap();
    assert_eq!(nft.traits, updated_traits);
}

#[test]
fn test_transfer() {
    let env = Env::default();
    let contract_id = env.register_contract(None, LivingNFTContract);
    let client = LivingNFTContractClient::new(&env, &contract_id);
    
    let oracle = Address::generate(&env);
    let owner = Address::generate(&env);
    let new_owner = Address::generate(&env);
    
    client.initialize(&oracle);
    
    let initial_traits = NFTTraits {
        background: 128,
        mood: 128,
        activity: 128,
        weather_effect: 128,
        time_of_day: 128,
        special_event: 0,
        power_level: 1000,
        rarity_score: 1000,
    };
    
    let token_id = client.mint_nft(
        &owner,
        &String::from_str(&env, "Test NFT"),
        &String::from_str(&env, "TEST"),
        &String::from_str(&env, "https://example.com/metadata.json"),
        &initial_traits,
    );
    
    client.transfer(&owner, &new_owner, &token_id);
    
    let nft = client.get_nft(&token_id).unwrap();
    assert_eq!(nft.owner, new_owner);
}
