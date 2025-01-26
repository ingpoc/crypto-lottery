use anchor_lang::prelude::*;
use crate::models::{state::*, error::*};

#[derive(Accounts)]
pub struct CleanupExpired<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        constraint = lottery.status != LotteryStatus::Completed && 
                    lottery.status != LotteryStatus::Cancelled
    )]
    pub lottery: Account<'info, LotteryAccount>,
    
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CleanupExpired>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = Clock::get()?;
    
    require!(
        clock.unix_timestamp > lottery.end_time,
        LotteryError::InvalidState
    );
    
    lottery.status = LotteryStatus::Cancelled;
    
    emit!(LotteryStateChanged {
        lottery: lottery.key(),
        old_state: LotteryStatus::Active.to_string(),
        new_state: LotteryStatus::Cancelled.to_string(),
        timestamp: clock.unix_timestamp,
    });
    
    Ok(())
}