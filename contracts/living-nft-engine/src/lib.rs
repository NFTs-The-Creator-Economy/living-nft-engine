use soroban_sdk::{contract, contractimpl, contracttype, Address, Bytes, Env, Symbol, Vec, Map, String};
use soroban_token_sdk::{TokenClient, TokenInterface};

// Contract state key
const DATA_KEY: Symbol = soroban_sdk::symbol!("DATA");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NFTTraits {
    pub background: u32,
    pub mood: u32,
    pub activity: u32,
    pub weather_effect: u32,
    pub time_of_day: u32,
    pub special_event: u32,
    pub power_level: u32,
    pub rarity_score: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LivingNFT {
    pub owner: Address,
    pub name: String,
    pub symbol: String,
    pub token_id: Bytes,
    pub metadata_uri: String,
    pub traits: NFTTraits,
    pub created_at: u64,
    pub updated_at: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OracleData {
    pub oracle_address: Address,
    pub is_authorized: bool,
    pub last_update: u64,
    pub update_count: u64,
}

#[contracttype]
pub enum DataKey {
    NFT(Bytes), // token_id -> NFT
    Oracle(Address), // oracle_address -> OracleData
    TotalSupply, // u64
    OwnerNFTs(Address), // owner -> Vec<token_id>
}

#[contract]
pub struct LivingNFTContract;

#[contractimpl]
impl LivingNFTContract {
    /// Initialize the contract with an authorized oracle
    pub fn initialize(env: Env, oracle_address: Address) {
        if env.storage().instance().has(&DATA_KEY) {
            panic!("already initialized");
        }

        // Initialize oracle data
        let oracle_data = OracleData {
            oracle_address: oracle_address.clone(),
            is_authorized: true,
            last_update: 0,
            update_count: 0,
        };
        
        env.storage().instance().set(&DataKey::Oracle(oracle_address), &oracle_data);
        env.storage().instance().set(&DataKey::TotalSupply, &0u64);
        env.storage().instance().set(&DATA_KEY, &true);
    }

    /// Mint a new Living NFT
    pub fn mint_nft(
        env: Env,
        owner: Address,
        name: String,
        symbol: String,
        metadata_uri: String,
        initial_traits: NFTTraits,
    ) -> Bytes {
        owner.require_auth();

        let token_id = self::generate_token_id(&env, &owner);
        let timestamp = env.ledger().timestamp();

        let nft = LivingNFT {
            owner: owner.clone(),
            name: name.clone(),
            symbol,
            token_id: token_id.clone(),
            metadata_uri,
            traits: initial_traits,
            created_at: timestamp,
            updated_at: timestamp,
            is_active: true,
        };

        // Store NFT
        env.storage().instance().set(&DataKey::NFT(token_id.clone()), &nft);
        
        // Update owner's NFT list
        let mut owner_nfts = env.storage().instance().get(&DataKey::OwnerNFTs(owner.clone())).unwrap_or(Vec::new(&env));
        owner_nfts.push_back(token_id.clone());
        env.storage().instance().set(&DataKey::OwnerNFTs(owner), &owner_nfts);

        // Update total supply
        let total_supply: u64 = env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0);
        env.storage().instance().set(&DataKey::TotalSupply, &(total_supply + 1));

        // Emit mint event
        env.events().publish(
            (Symbol::short("MINT"), owner, name),
            token_id.clone(),
        );

        token_id
    }

    /// Update NFT traits (oracle only)
    pub fn update_traits(
        env: Env,
        oracle: Address,
        token_id: Bytes,
        new_traits: NFTTraits,
    ) {
        // Verify oracle authorization
        let oracle_data: OracleData = env.storage().instance()
            .get(&DataKey::Oracle(oracle.clone()))
            .unwrap_or_else(|| panic!("oracle not found"));
        
        if !oracle_data.is_authorized {
            panic!("unauthorized oracle");
        }

        oracle.require_auth();

        // Get existing NFT
        let mut nft: LivingNFT = env.storage().instance()
            .get(&DataKey::NFT(token_id.clone()))
            .unwrap_or_else(|| panic!("nft not found"));

        nft.traits = new_traits;
        nft.updated_at = env.ledger().timestamp();

        // Store updated NFT
        env.storage().instance().set(&DataKey::NFT(token_id.clone()), &nft);

        // Update oracle stats
        let updated_oracle = OracleData {
            last_update: env.ledger().timestamp(),
            update_count: oracle_data.update_count + 1,
            ..oracle_data
        };
        env.storage().instance().set(&DataKey::Oracle(oracle), &updated_oracle);

        // Emit update event
        env.events().publish(
            (Symbol::short("UPDATE"), token_id),
            nft.traits,
        );
    }

    /// Get NFT by token ID
    pub fn get_nft(env: Env, token_id: Bytes) -> Option<LivingNFT> {
        env.storage().instance().get(&DataKey::NFT(token_id))
    }

    /// Get all NFTs owned by an address
    pub fn get_owner_nfts(env: Env, owner: Address) -> Vec<LivingNFT> {
        let token_ids: Vec<Bytes> = env.storage().instance()
            .get(&DataKey::OwnerNFTs(owner))
            .unwrap_or(Vec::new(&env));

        let mut nfts = Vec::new(&env);
        for token_id in token_ids.iter() {
            if let Some(nft) = env.storage().instance().get(&DataKey::NFT(token_id)) {
                nfts.push_back(nft);
            }
        }
        nfts
    }

    /// Get total supply
    pub fn get_total_supply(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::TotalSupply).unwrap_or(0)
    }

    /// Add or authorize an oracle
    pub fn authorize_oracle(env: Env, admin: Address, oracle_address: Address) {
        // For simplicity, we'll use the first oracle as admin
        let first_oracle_key = DataKey::Oracle(admin.clone());
        let admin_data: OracleData = env.storage().instance()
            .get(&first_oracle_key)
            .unwrap_or_else(|| panic!("admin not found"));

        admin.require_auth();

        let oracle_data = OracleData {
            oracle_address: oracle_address.clone(),
            is_authorized: true,
            last_update: 0,
            update_count: 0,
        };
        
        env.storage().instance().set(&DataKey::Oracle(oracle_address), &oracle_data);
    }

    /// Deauthorize an oracle
    pub fn deauthorize_oracle(env: Env, admin: Address, oracle_address: Address) {
        let first_oracle_key = DataKey::Oracle(admin.clone());
        let admin_data: OracleData = env.storage().instance()
            .get(&first_oracle_key)
            .unwrap_or_else(|| panic!("admin not found"));

        admin.require_auth();

        let mut oracle_data: OracleData = env.storage().instance()
            .get(&DataKey::Oracle(oracle_address.clone()))
            .unwrap_or_else(|| panic!("oracle not found"));
        
        oracle_data.is_authorized = false;
        env.storage().instance().set(&DataKey::Oracle(oracle_address), &oracle_data);
    }

    /// Get oracle information
    pub fn get_oracle_info(env: Env, oracle_address: Address) -> Option<OracleData> {
        env.storage().instance().get(&DataKey::Oracle(oracle_address))
    }

    /// Transfer NFT ownership
    pub fn transfer(env: Env, from: Address, to: Address, token_id: Bytes) {
        from.require_auth();

        // Get and remove NFT from current owner
        let mut nft: LivingNFT = env.storage().instance()
            .get(&DataKey::NFT(token_id.clone()))
            .unwrap_or_else(|| panic!("nft not found"));

        if nft.owner != from {
            panic!("not owner");
        }

        // Remove from current owner's list
        let mut from_nfts: Vec<Bytes> = env.storage().instance()
            .get(&DataKey::OwnerNFTs(from.clone()))
            .unwrap_or(Vec::new(&env));
        
        let index = from_nfts.iter().position(|id| id == &token_id)
            .unwrap_or_else(|| panic!("nft not in owner list"));
        from_nfts.remove(index as u32);
        env.storage().instance().set(&DataKey::OwnerNFTs(from), &from_nfts);

        // Add to new owner's list
        let mut to_nfts: Vec<Bytes> = env.storage().instance()
            .get(&DataKey::OwnerNFTs(to.clone()))
            .unwrap_or(Vec::new(&env));
        to_nfts.push_back(token_id.clone());
        env.storage().instance().set(&DataKey::OwnerNFTs(to.clone()), &to_nfts);

        // Update NFT owner
        nft.owner = to.clone();
        nft.updated_at = env.ledger().timestamp();
        env.storage().instance().set(&DataKey::NFT(token_id.clone()), &nft);

        // Emit transfer event
        env.events().publish(
            (Symbol::short("TRANSFER"), from, to),
            token_id,
        );
    }
}

// Helper function to generate unique token ID
fn generate_token_id(env: &Env, owner: &Address) -> Bytes {
    let timestamp = env.ledger().timestamp();
    let mut data = Bytes::new(env);
    data.extend_from_slice(owner.to_bytes().as_slice());
    data.extend_from_slice(&timestamp.to_be_bytes());
    data.extend_from_slice(&env.crypto().sha256(&data));
    data
}

#[cfg(test)]
mod test;
