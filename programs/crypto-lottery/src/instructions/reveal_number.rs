use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct RevealNumber<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(mut)]
    pub lottery: Account<'info, LotteryAccount>,
}

pub fn handler(ctx: Context<RevealNumber>) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = Clock::get()?;
    
    let recent_slothash = clock.slot.to_le_bytes();
    let mut hasher = std::hash::DefaultHasher::new();
    hasher.write(&recent_slothash);
    let random = hasher.finish();
    
    lottery.winning_number = Some(random % 1_000_000); // 6-digit number
    lottery.status = LotteryStatus::Completed;

    Ok(())
}