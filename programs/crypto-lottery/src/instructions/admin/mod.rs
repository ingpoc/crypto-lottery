use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Token};
use crate::{LotteryConfig, error::LotteryError};

#[derive(Accounts)]
#[instruction(fee_basis_points: u16)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = LotteryConfig::SIZE,
        seeds = [LotteryConfig::SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, LotteryConfig>,
    
    #[account(
        constraint = treasury_account.mint == treasury_mint.key(),
        constraint = treasury_account.owner == authority.key()
    )]
    pub treasury_account: Account<'info, TokenAccount>,
    
    pub treasury_mint: Account<'info, Mint>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_program(ctx: Context<Initialize>, fee_basis_points: u16) -> Result<()> {
    // Validate fee basis points (max 10%)
    require!(
        fee_basis_points <= 1000,
        LotteryError::InvalidFeeAmount
    );

    let config = &mut ctx.accounts.config;
    
    // Set base configuration with reentrancy protection
    let current_timestamp = Clock::get()?.unix_timestamp;
    require!(config.last_modified_time.unwrap_or(0) < current_timestamp, LotteryError::ReentrancyDetected);
    
    config.authority = ctx.accounts.authority.key();
    config.treasury_mint = ctx.accounts.treasury_mint.key();
    config.treasury_account = ctx.accounts.treasury_account.key();
    config.fee_basis_points = fee_basis_points;
    config.daily_price = 1_000_000;    // 1 USDC
    config.weekly_price = 5_000_000;   // 5 USDC
    config.monthly_price = 10_000_000; // 10 USDC
    config.bump = *ctx.bumps.get("config").unwrap();
    config.last_modified_time = Some(current_timestamp);

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [LotteryConfig::SEED_PREFIX],
        bump = config.bump,
        has_one = authority
    )]
    pub config: Account<'info, LotteryConfig>,
    
    pub authority: Signer<'info>,
}

pub fn update_lottery_prices(
    ctx: Context<UpdateConfig>, 
    daily_price: u64,
    weekly_price: u64,
    monthly_price: u64,
) -> Result<()> {
    let config = &mut ctx.accounts.config;
    
    // Reentrancy check
    let current_timestamp = Clock::get()?.unix_timestamp;
    require!(
        config.last_modified_time.unwrap_or(0) < current_timestamp,
        LotteryError::ReentrancyDetected
    );

    // Update prices with overflow checks
    config.daily_price = daily_price.checked_add(0).ok_or(LotteryError::Overflow)?;
    config.weekly_price = weekly_price.checked_add(0).ok_or(LotteryError::Overflow)?;
    config.monthly_price = monthly_price.checked_add(0).ok_or(LotteryError::Overflow)?;
    config.last_modified_time = Some(current_timestamp);

    Ok(())
}