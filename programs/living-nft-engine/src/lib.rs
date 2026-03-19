use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("11111111111111111111111111111111");

#[program]
pub mod living_nft_engine {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, oracle_authority: Pubkey) -> Result<()> {
        let config = &mut ctx.accounts.config;
        config.oracle_authority = oracle_authority;
        config.bump = ctx.bumps.config;
        Ok(())
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        name: String,
        symbol: String,
        uri: String,
        initial_traits: NFTTraits,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        nft.owner = ctx.accounts.owner.key();
        nft.mint = ctx.accounts.mint.key();
        nft.name = name;
        nft.symbol = symbol;
        nft.uri = uri;
        nft.traits = initial_traits;
        nft.last_updated = Clock::get()?.unix_timestamp;
        nft.bump = ctx.bumps.nft;

        // Transfer NFT to owner
        let cpi_accounts = Transfer {
            from: ctx.accounts.token_account.to_account_info(),
            to: ctx.accounts.owner_token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, 1)?;

        Ok(())
    }

    pub fn update_nft_traits(
        ctx: Context<UpdateNFTTraits>,
        new_traits: NFTTraits,
        new_uri: Option<String>,
    ) -> Result<()> {
        let config = &ctx.accounts.config;
        
        // Verify caller is the authorized oracle
        require!(
            ctx.accounts.oracle.key() == config.oracle_authority,
            ErrorCode::UnauthorizedOracle
        );

        let nft = &mut ctx.accounts.nft;
        nft.traits = new_traits;
        if let Some(uri) = new_uri {
            nft.uri = uri;
        }
        nft.last_updated = Clock::get()?.unix_timestamp;

        emit!(NFTTraitsUpdated {
            nft: nft.key(),
            owner: nft.owner,
            new_traits: nft.traits.clone(),
            updated_at: nft.last_updated,
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(oracle_authority: Pubkey)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + Config::INIT_SPACE,
        seeds = [b"config"],
        bump
    )]
    pub config: Account<'info, Config>,
    
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + NFT::INIT_SPACE,
        seeds = [b"nft", mint.key().as_ref()],
        bump
    )]
    pub nft: Account<'info, NFT>,
    
    #[account(
        init,
        payer = owner,
        mint::decimals = 0,
        mint::authority = mint_authority,
        mint::freeze_authority = mint_authority,
    )]
    pub mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = owner,
        token::mint = mint,
        token::authority = mint_authority,
    )]
    pub token_account: Account<'info, TokenAccount>,
    
    #[account(
        init_if_needed,
        payer = owner,
        token::mint = mint,
        token::authority = owner,
    )]
    pub owner_token_account: Account<'info, TokenAccount>,
    
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct UpdateNFTTraits<'info> {
    pub config: Account<'info, Config>,
    
    #[account(mut)]
    pub nft: Account<'info, NFT>,
    
    pub oracle: Signer<'info>,
}

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub oracle_authority: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct NFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    #[max_len(50)]
    pub name: String,
    #[max_len(10)]
    pub symbol: String,
    #[max_len(200)]
    pub uri: String,
    pub traits: NFTTraits,
    pub last_updated: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct NFTTraits {
    pub background: u8,      // 0-255: represents different background colors/scenes
    pub mood: u8,           // 0-255: happy, sad, excited, etc.
    pub activity: u8,       // 0-255: sleeping, playing, working, etc.
    pub weather_effect: u8,  // 0-255: sunny, rainy, snowy, etc.
    pub time_of_day: u8,    // 0-255: morning, afternoon, evening, night
    pub special_event: u8,  // 0-255: holiday, celebration, etc.
    pub power_level: u16,   // 0-65535: strength based on external factors
    pub rarity_score: u16,  // 0-65535: calculated rarity score
}

#[event]
pub struct NFTTraitsUpdated {
    pub nft: Pubkey,
    pub owner: Pubkey,
    pub new_traits: NFTTraits,
    pub updated_at: i64,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Only the authorized oracle can update NFT traits")]
    UnauthorizedOracle,
}
