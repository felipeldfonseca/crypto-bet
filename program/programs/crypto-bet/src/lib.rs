use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use solana_program::pubkey;

// Use the correct program ID from Anchor.toml
declare_id!("4Gd64thyhLeqyLxDz8Ae5Z98qXdqwJrcAYkS6g3Yzy5V");

// Constants for supported tokens
pub const SOL_MINT: Pubkey = Pubkey::new_from_array([0; 32]); // Native SOL (placeholder)
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"); // USDC mainnet

#[program]
pub mod crypto_bet {
    use super::*;

    /// Initialize a new prediction market with dual-mode support
    /// Only the program authority can create markets
    pub fn initialize_market(
        ctx: Context<InitializeMarket>,
        market_id: u64,
        title: String,
        description: String,
        category: String,
        resolution_time: i64,
        market_type: MarketType,
    ) -> Result<()> {
        require!(title.len() <= 100, CryptoBetError::TitleTooLong);
        require!(description.len() <= 500, CryptoBetError::DescriptionTooLong);
        require!(category.len() <= 50, CryptoBetError::CategoryTooLong);
        require!(resolution_time > Clock::get()?.unix_timestamp, CryptoBetError::InvalidResolutionTime);

        let max_duration = 365 * 24 * 60 * 60; // 1 year in seconds
        require!(
            resolution_time <= Clock::get()?.unix_timestamp + max_duration,
            CryptoBetError::MarketDurationTooLong
        );

        let market = &mut ctx.accounts.market;
        market.authority = ctx.accounts.authority.key();
        market.market_id = market_id;
        market.title = title;
        market.description = description;
        market.category = category;
        market.resolution_time = resolution_time;
        market.state = MarketState::Active;
        market.market_type = market_type;
        
        // Set accepted mint based on market type
        market.accepted_mint = match market_type {
            MarketType::Degen => SOL_MINT,
            MarketType::Stable => USDC_MINT,
        };
        
        market.total_yes_amount = 0;
        market.total_no_amount = 0;
        market.total_yes_shares = 0;
        market.total_no_shares = 0;
        market.total_volume = 0;
        market.resolved_outcome = None;
        market.created_at = Clock::get()?.unix_timestamp;
        market.bump = ctx.bumps.market;

        emit!(MarketCreated {
            market: market.key(),
            market_id,
            title: market.title.clone(),
            category: market.category.clone(),
            resolution_time,
            market_type,
        });

        Ok(())
    }

    /// Place a bet on a market using dual-mode mechanics (SOL or USDC)
    /// Users can bet YES or NO on the market outcome
    pub fn place_bet(
        ctx: Context<PlaceBet>,
        side: BetSide,
        amount: u64,
    ) -> Result<()> {
        require!(amount > 0, CryptoBetError::InvalidAmount);
        require!(amount >= MIN_BET_AMOUNT, CryptoBetError::BetTooSmall);
        require!(amount <= MAX_BET_AMOUNT, CryptoBetError::BetTooLarge);

        let market = &mut ctx.accounts.market;
        require!(market.state == MarketState::Active, CryptoBetError::MarketNotActive);
        require!(Clock::get()?.unix_timestamp < market.resolution_time, CryptoBetError::MarketExpired);

        let new_total_volume = market.total_volume
            .checked_add(amount)
            .ok_or(CryptoBetError::MathOverflow)?;
        
        require!(new_total_volume <= MAX_MARKET_VOLUME, CryptoBetError::MarketVolumeTooHigh);

        // For prediction markets, use 1:1 share ratio (1 token = 1 share)
        let shares = amount;

        // Handle transfer based on market type
        match market.market_type {
            MarketType::Degen => {
                // SOL transfer (native)
                let cpi_context = CpiContext::new(
                    ctx.accounts.system_program.to_account_info(),
                    anchor_lang::system_program::Transfer {
                        from: ctx.accounts.user.to_account_info(),
                        to: ctx.accounts.market.to_account_info(),
                    },
                );
                anchor_lang::system_program::transfer(cpi_context, amount)?;
            }
            MarketType::Stable => {
                // USDC transfer (SPL token)
                let user_token_account = ctx.accounts.user_token_account
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenAccount)?;
                let market_vault = ctx.accounts.market_vault
                    .as_ref()
                    .ok_or(CryptoBetError::MissingVault)?;
                let token_program = ctx.accounts.token_program
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenProgram)?;

                // Verify token account mint matches market's accepted mint
                require!(
                    user_token_account.mint == market.accepted_mint,
                    CryptoBetError::InvalidTokenMint
                );
                require!(
                    market_vault.mint == market.accepted_mint,
                    CryptoBetError::InvalidTokenMint
                );

                let cpi_accounts = Transfer {
                    from: user_token_account.to_account_info(),
                    to: market_vault.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                };
                let cpi_ctx = CpiContext::new(token_program.to_account_info(), cpi_accounts);
                token::transfer(cpi_ctx, amount)?;
            }
        }

        // Update market totals (same logic for both modes)
        match side {
            BetSide::Yes => {
                market.total_yes_amount = market.total_yes_amount
                    .checked_add(amount)
                    .ok_or(CryptoBetError::MathOverflow)?;
                market.total_yes_shares = market.total_yes_shares
                    .checked_add(shares)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
            BetSide::No => {
                market.total_no_amount = market.total_no_amount
                    .checked_add(amount)
                    .ok_or(CryptoBetError::MathOverflow)?;
                market.total_no_shares = market.total_no_shares
                    .checked_add(shares)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
        }
        market.total_volume = new_total_volume;

        // Initialize or update user position
        let position = &mut ctx.accounts.position;
        if position.user == Pubkey::default() {
            // First time betting on this market
            position.user = ctx.accounts.user.key();
            position.market = market.key();
            position.yes_shares = 0;
            position.no_shares = 0;
            position.total_invested = 0;
            position.claimed = false;
            position.bump = ctx.bumps.position;
        }

        // Update position
        match side {
            BetSide::Yes => {
                position.yes_shares = position.yes_shares
                    .checked_add(shares)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
            BetSide::No => {
                position.no_shares = position.no_shares
                    .checked_add(shares)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
        }
        position.total_invested = position.total_invested
            .checked_add(amount)
            .ok_or(CryptoBetError::MathOverflow)?;

        emit!(BetPlaced {
            market: market.key(),
            user: ctx.accounts.user.key(),
            side,
            amount,
            shares,
            new_yes_total: market.total_yes_amount,
            new_no_total: market.total_no_amount,
            market_type: market.market_type,
        });

        Ok(())
    }

    /// Resolve a market with the final outcome
    /// Only the market authority can resolve markets
    pub fn resolve_market(
        ctx: Context<ResolveMarket>,
        outcome: bool,
    ) -> Result<()> {
        let market = &mut ctx.accounts.market;
        require!(market.state == MarketState::Active, CryptoBetError::MarketNotActive);
        require!(Clock::get()?.unix_timestamp >= market.resolution_time, CryptoBetError::MarketNotExpired);
        require!(market.authority == ctx.accounts.authority.key(), CryptoBetError::UnauthorizedResolver);

        market.state = MarketState::Resolved;
        market.resolved_outcome = Some(outcome);

        emit!(MarketResolved {
            market: market.key(),
            outcome,
            total_volume: market.total_volume,
            market_type: market.market_type,
        });

        Ok(())
    }

    /// Cancel a market before resolution
    /// Only the market authority can cancel markets
    pub fn cancel_market(ctx: Context<CancelMarket>) -> Result<()> {
        let market = &mut ctx.accounts.market;
        require!(market.state == MarketState::Active, CryptoBetError::MarketNotActive);
        require!(market.authority == ctx.accounts.authority.key(), CryptoBetError::UnauthorizedResolver);

        market.state = MarketState::Cancelled;

        emit!(MarketCancelled {
            market: market.key(),
            total_volume: market.total_volume,
            market_type: market.market_type,
        });

        Ok(())
    }

    /// Claim winnings from a resolved market (dual-mode support)
    /// Users can claim their share of the winning pool
    pub fn claim_winnings(ctx: Context<ClaimWinnings>) -> Result<()> {
        let market = &ctx.accounts.market;
        let position = &mut ctx.accounts.position;

        require!(market.state == MarketState::Resolved, CryptoBetError::MarketNotResolved);
        require!(position.user == ctx.accounts.user.key(), CryptoBetError::InvalidPosition);

        require!(!position.claimed, CryptoBetError::AlreadyClaimed);

        let resolved_outcome = market.resolved_outcome.ok_or(CryptoBetError::MarketNotResolved)?;
        
        // Calculate winnings based on the resolved outcome
        let winning_shares = if resolved_outcome {
            position.yes_shares
        } else {
            position.no_shares
        };

        require!(winning_shares > 0, CryptoBetError::NoWinningShares);

        // Use total pool for winnings calculation
        let total_winning_pool = market.total_yes_amount
            .checked_add(market.total_no_amount)
            .ok_or(CryptoBetError::MathOverflow)?;

        // Use correct share totals for calculation
        let total_winning_shares = if resolved_outcome {
            market.total_yes_shares
        } else {
            market.total_no_shares
        };

        require!(total_winning_shares > 0, CryptoBetError::NoWinningShares);

        // Calculate proportional winnings: (user_winning_shares / total_winning_shares) * total_pool
        let winnings = (winning_shares as u128)
            .checked_mul(total_winning_pool as u128)
            .ok_or(CryptoBetError::MathOverflow)?
            .checked_div(total_winning_shares as u128)
            .ok_or(CryptoBetError::DivisionByZero)? as u64;

        require!(winnings > 0, CryptoBetError::NoWinningsAvailable);

        // Use correct share totals for calculation
        let total_winning_shares = if resolved_outcome {
            market.total_yes_shares
        } else {
            market.total_no_shares
        };

        require!(total_winning_shares > 0, CryptoBetError::NoWinningShares);

        // Calculate proportional winnings: (user_winning_shares / total_winning_shares) * total_pool
        let winnings = (winning_shares as u128)
            .checked_mul(total_winning_pool as u128)
            .ok_or(CryptoBetError::MathOverflow)?
            .checked_div(total_winning_shares as u128)
            .ok_or(CryptoBetError::DivisionByZero)? as u64;

        require!(winnings > 0, CryptoBetError::NoWinningsAvailable);

        // Prevent double claiming
        require!(
            (resolved_outcome && position.yes_shares > 0) || (!resolved_outcome && position.no_shares > 0),
            CryptoBetError::AlreadyClaimed
        );

        // Transfer winnings based on market type
        match market.market_type {
            MarketType::Degen => {
                // SOL transfer from market to user
                **market.to_account_info().try_borrow_mut_lamports()? = market
                    .to_account_info()
                    .lamports()
                    .checked_sub(winnings)
                    .ok_or(CryptoBetError::InsufficientFunds)?;

                **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? = ctx
                    .accounts
                    .user
                    .to_account_info()
                    .lamports()
                    .checked_add(winnings)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
            MarketType::Stable => {
                // USDC transfer from vault to user
                let user_token_account = ctx.accounts.user_token_account
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenAccount)?;
                let market_vault = ctx.accounts.market_vault
                    .as_ref()
                    .ok_or(CryptoBetError::MissingVault)?;
                let token_program = ctx.accounts.token_program
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenProgram)?;

                // Create PDA signer for vault
                let market_key = market.key();
                let seeds = &[
                    b"vault",
                    market_key.as_ref(),
                    &[ctx.bumps.market_vault.unwrap_or(0)]
                ];
                let signer = &[&seeds[..]];

                let cpi_accounts = Transfer {
                    from: market_vault.to_account_info(),
                    to: user_token_account.to_account_info(),
                    authority: market_vault.to_account_info(), // Vault is its own authority
                };
                let cpi_ctx = CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    cpi_accounts,
                    signer
                );
                token::transfer(cpi_ctx, winnings)?;
            }
        }

        // Mark position as claimed to prevent double claiming
        if resolved_outcome {
            position.yes_shares = 0;
        } else {
            position.no_shares = 0;
        }

        position.claimed = true;

        emit!(WinningsClaimed {
            market: market.key(),
            user: ctx.accounts.user.key(),
            amount: winnings,
            market_type: market.market_type,
        });

        Ok(())
    }

    /// Claim refund from a cancelled market (dual-mode support)
    /// Users can get their original bet amount back
    pub fn claim_refund(ctx: Context<ClaimRefund>) -> Result<()> {
        let market = &ctx.accounts.market;
        let position = &mut ctx.accounts.position;

        require!(market.state == MarketState::Cancelled, CryptoBetError::MarketNotCancelled);
        require!(position.user == ctx.accounts.user.key(), CryptoBetError::InvalidPosition);
        require!(position.total_invested > 0, CryptoBetError::NoRefundAvailable);

        require!(!position.claimed, CryptoBetError::AlreadyClaimed);

        let refund_amount = position.total_invested;

        // Transfer refund based on market type
        match market.market_type {
            MarketType::Degen => {
                // SOL refund
                **market.to_account_info().try_borrow_mut_lamports()? = market
                    .to_account_info()
                    .lamports()
                    .checked_sub(refund_amount)
                    .ok_or(CryptoBetError::InsufficientFunds)?;

                **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? = ctx
                    .accounts
                    .user
                    .to_account_info()
                    .lamports()
                    .checked_add(refund_amount)
                    .ok_or(CryptoBetError::MathOverflow)?;
            }
            MarketType::Stable => {
                // USDC transfer from vault to user
                let user_token_account = ctx.accounts.user_token_account
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenAccount)?;
                let market_vault = ctx.accounts.market_vault
                    .as_ref()
                    .ok_or(CryptoBetError::MissingVault)?;
                let token_program = ctx.accounts.token_program
                    .as_ref()
                    .ok_or(CryptoBetError::MissingTokenProgram)?;

                // Create PDA signer for vault
                let market_key = market.key();
                let seeds = &[
                    b"vault",
                    market_key.as_ref(),
                    &[ctx.bumps.market_vault.unwrap_or(0)]
                ];
                let signer = &[&seeds[..]];

                let cpi_accounts = Transfer {
                    from: market_vault.to_account_info(),
                    to: user_token_account.to_account_info(),
                    authority: market_vault.to_account_info(),
                };
                let cpi_ctx = CpiContext::new_with_signer(
                    token_program.to_account_info(),
                    cpi_accounts,
                    signer
                );
                token::transfer(cpi_ctx, refund_amount)?;
            }
        }

        // Mark position as refunded
        position.yes_shares = 0;
        position.no_shares = 0;
        position.total_invested = 0;

        position.claimed = true;

        emit!(RefundClaimed {
            market: market.key(),
            user: ctx.accounts.user.key(),
            amount: refund_amount,
            market_type: market.market_type,
        });

        Ok(())
    }
}

// Context Structs

#[derive(Accounts)]
#[instruction(market_id: u64, market_type: MarketType)]
pub struct InitializeMarket<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + Market::LEN,
        seeds = [b"market", &market_id.to_le_bytes()],
        bump
    )]
    pub market: Account<'info, Market>,
    
    // Optional vault for USDC markets
    #[account(
        init_if_needed,
        payer = authority,
        token::mint = usdc_mint,
        token::authority = market,
        seeds = [b"vault", market.key().as_ref()],
        bump
    )]
    pub market_vault: Option<Account<'info, TokenAccount>>,
    
    /// CHECK: USDC mint address
    pub usdc_mint: Option<AccountInfo<'info>>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Option<Program<'info, Token>>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        mut,
        seeds = [b"market", &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + Position::LEN,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Optional accounts for USDC markets
    #[account(mut)]
    pub user_token_account: Option<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [b"vault", market.key().as_ref()],
        bump
    )]
    pub market_vault: Option<Account<'info, TokenAccount>>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Option<Program<'info, Token>>,
}

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(
        mut,
        seeds = [b"market", &market.market_id.to_le_bytes()],
        bump = market.bump,
        has_one = authority
    )]
    pub market: Account<'info, Market>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CancelMarket<'info> {
    #[account(
        mut,
        seeds = [b"market", &market.market_id.to_le_bytes()],
        bump = market.bump,
        has_one = authority
    )]
    pub market: Account<'info, Market>,
    
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(
        seeds = [b"market", &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        mut,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump = position.bump,
        has_one = user
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Optional accounts for USDC markets
    #[account(mut)]
    pub user_token_account: Option<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [b"vault", market.key().as_ref()],
        bump
    )]
    pub market_vault: Option<Account<'info, TokenAccount>>,
    
    pub token_program: Option<Program<'info, Token>>,
}

#[derive(Accounts)]
pub struct ClaimRefund<'info> {
    #[account(
        seeds = [b"market", &market.market_id.to_le_bytes()],
        bump = market.bump
    )]
    pub market: Account<'info, Market>,
    
    #[account(
        mut,
        seeds = [b"position", market.key().as_ref(), user.key().as_ref()],
        bump = position.bump,
        has_one = user
    )]
    pub position: Account<'info, Position>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    // Optional accounts for USDC markets
    #[account(mut)]
    pub user_token_account: Option<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [b"vault", market.key().as_ref()],
        bump
    )]
    pub market_vault: Option<Account<'info, TokenAccount>>,
    
    pub token_program: Option<Program<'info, Token>>,
}

// Account Structs

#[account]
pub struct Market {
    pub authority: Pubkey,          // 32
    pub market_id: u64,             // 8
    pub title: String,              // 4 + 100
    pub description: String,        // 4 + 500
    pub category: String,           // 4 + 50
    pub resolution_time: i64,       // 8
    pub state: MarketState,         // 1
    pub market_type: MarketType,    // 1 - NEW: Degen vs Stable
    pub accepted_mint: Pubkey,      // 32 - NEW: SOL or USDC mint
    pub total_yes_amount: u64,      // 8 - Token amounts
    pub total_no_amount: u64,       // 8 - Token amounts
    pub total_yes_shares: u64,      // 8 - Share counts
    pub total_no_shares: u64,       // 8 - Share counts
    pub total_volume: u64,          // 8
    pub resolved_outcome: Option<bool>, // 1 + 1
    pub created_at: i64,            // 8
    pub bump: u8,                   // 1
}

impl Market {
    pub const LEN: usize = 32 + 8 + (4 + 100) + (4 + 500) + (4 + 50) + 8 + 1 + 1 + 32 + 8 + 8 + 8 + 8 + 8 + 2 + 8 + 1;
}

#[account]
pub struct Position {
    pub user: Pubkey,           // 32
    pub market: Pubkey,         // 32
    pub yes_shares: u64,        // 8
    pub no_shares: u64,         // 8
    pub total_invested: u64,    // 8
    pub claimed: bool,          // 1 - 🔒 SECURITY: Track claim status to prevent double claiming
    pub bump: u8,               // 1
}

impl Position {
    pub const LEN: usize = 32 + 32 + 8 + 8 + 8 + 1 + 1; // Updated size
}

// Enums and Types

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MarketState {
    Active,
    Resolved,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum MarketType {
    Degen,      // SOL betting
    Stable,     // USDC betting
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq)]
pub enum BetSide {
    Yes,
    No,
}

// Events

#[event]
pub struct MarketCreated {
    pub market: Pubkey,
    pub market_id: u64,
    pub title: String,
    pub category: String,
    pub resolution_time: i64,
    pub market_type: MarketType,
}

#[event]
pub struct BetPlaced {
    pub market: Pubkey,
    pub user: Pubkey,
    pub side: BetSide,
    pub amount: u64,
    pub shares: u64,
    pub new_yes_total: u64,
    pub new_no_total: u64,
    pub market_type: MarketType,
}

#[event]
pub struct MarketResolved {
    pub market: Pubkey,
    pub outcome: bool,
    pub total_volume: u64,
    pub market_type: MarketType,
}

#[event]
pub struct MarketCancelled {
    pub market: Pubkey,
    pub total_volume: u64,
    pub market_type: MarketType,
}

#[event]
pub struct WinningsClaimed {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub market_type: MarketType,
}

#[event]
pub struct RefundClaimed {
    pub market: Pubkey,
    pub user: Pubkey,
    pub amount: u64,
    pub market_type: MarketType,
}

// Error Codes

#[error_code]
pub enum CryptoBetError {
    #[msg("Title is too long (max 100 characters)")]
    TitleTooLong,
    #[msg("Description is too long (max 500 characters)")]
    DescriptionTooLong,
    #[msg("Category is too long (max 50 characters)")]
    CategoryTooLong,
    #[msg("Resolution time must be in the future")]
    InvalidResolutionTime,
    #[msg("Market duration cannot exceed 1 year")]
    MarketDurationTooLong,
    #[msg("Invalid bet amount")]
    InvalidAmount,
    #[msg("Bet amount is too small")]
    BetTooSmall,
    #[msg("Bet amount is too large")]
    BetTooLarge,
    #[msg("Market volume too high")]
    MarketVolumeTooHigh,
    #[msg("Market is not active")]
    MarketNotActive,
    #[msg("Market has expired")]
    MarketExpired,
    #[msg("Market has not expired yet")]
    MarketNotExpired,
    #[msg("Market is not resolved")]
    MarketNotResolved,
    #[msg("Market is not cancelled")]
    MarketNotCancelled,
    #[msg("Math overflow")]
    MathOverflow,
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Unauthorized resolver")]
    UnauthorizedResolver,
    #[msg("Invalid position")]
    InvalidPosition,
    #[msg("No winning shares")]
    NoWinningShares,
    #[msg("No winnings available")]
    NoWinningsAvailable,
    #[msg("No refund available")]
    NoRefundAvailable,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Winnings or refund already claimed")]
    AlreadyClaimed,
    #[msg("Missing token account")]
    MissingTokenAccount,
    #[msg("Missing market vault")]
    MissingVault,
    #[msg("Missing token program")]
    MissingTokenProgram,
    #[msg("Invalid token mint")]
    InvalidTokenMint,
}

// Constants

pub const MIN_BET_AMOUNT: u64 = 1_000_000; // 0.001 SOL or 0.001 USDC
pub const MAX_BET_AMOUNT: u64 = 1_000_000_000_000; // 1,000 SOL or 1,000 USDC
pub const MAX_MARKET_VOLUME: u64 = 10_000_000_000_000; // 10,000 SOL or 10,000 USDC 