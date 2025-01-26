use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Token},
    associated_token::AssociatedToken,
};
use crate::{state::*, models::constants::*};

#[derive(Accounts)]
#[instruction(lottery_type: LotteryType)]
pub struct CreateLottery<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        seeds = [LotteryConfig::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, LotteryConfig>,
    
    #[account(
        init,
        payer = authority,
        space = LotteryAccount::SIZE,
        seeds = [
            LotteryAccount::SEED_PREFIX,
            lottery_type.to_string().as_bytes(),
            Clock::get()?.unix_timestamp.to_le_bytes().as_ref()
        ],
        bump
    )]
    pub lottery: Account<'info, LotteryAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

pub fn handler(ctx: Context<CreateLottery>, lottery_type: LotteryType) -> Result<()> {
    let lottery = &mut ctx.accounts.lottery;
    let clock = Clock::get()?;
    
    lottery.lottery_type = lottery_type;
    lottery.status = LotteryStatus::Active;
    lottery.start_time = clock.unix_timestamp;
    lottery.end_time = clock.unix_timestamp + get_lottery_duration(lottery_type);
    lottery.ticket_price = get_ticket_price(lottery_type, &ctx.accounts.config);
    lottery.total_tickets = 0;
    lottery.prize_pool = 0;
    lottery.bump = *ctx.bumps.get("lottery").unwrap();

    Ok(())
}