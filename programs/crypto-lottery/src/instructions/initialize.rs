use anchor_lang::prelude::*;
use crate::{Initialize, models::constants};

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let clock = Clock::get()?;

    config.authority = ctx.accounts.authority.key();
    config.last_modified_time = clock.unix_timestamp;
    config.fee_basis_points = constants::TREASURY_FEE;
    config.daily_price = constants::DAILY_LOTTERY_PRICE;
    config.weekly_price = constants::WEEKLY_LOTTERY_PRICE;
    config.monthly_price = constants::MONTHLY_LOTTERY_PRICE;

    Ok(())
}