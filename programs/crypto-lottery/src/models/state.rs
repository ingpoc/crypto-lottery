use anchor_lang::prelude::*;
use std::fmt;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LotteryType {
    Daily,
    Weekly,
    Monthly,
}

impl fmt::Display for LotteryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, Debug)]
pub enum LotteryStatus {
    Pending,
    Active,
    NumberRevealing,
    Completed,
    Cancelled,
}

impl fmt::Display for LotteryStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[account]
#[derive(Debug)]
pub struct ProgramConfig {
    pub authority: Pubkey,
    pub treasury_mint: Pubkey,
    pub treasury_account: Pubkey,
    pub fee_basis_points: u16,
    pub daily_price: u64,
    pub weekly_price: u64,
    pub monthly_price: u64,
    pub last_modified_time: i64,
    pub bump: u8,
}

impl ProgramConfig {
    pub const SPACE: usize = 32 + // authority
                            32 + // treasury_mint
                            32 + // treasury_account
                            2 +  // fee_basis_points
                            8 +  // daily_price
                            8 +  // weekly_price
                            8 +  // monthly_price
                            8 +  // last_modified_time
                            1;   // bump
}

#[account]
#[derive(Debug)]
pub struct LotteryAccount {
    pub lottery_type: LotteryType,
    pub status: LotteryStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub ticket_price: u64,
    pub total_tickets: u64,
    pub prize_pool: u64,
    pub winning_number: Option<u64>,
    pub bump: u8,
}

#[account]
#[derive(Debug)]
pub struct TicketAccount {
    pub ticket_number: u64,
    pub owner: Pubkey,
    pub lottery: Pubkey,
    pub is_winner: bool,
    pub claimed: bool,
    pub prize_amount: u64,
    pub bump: u8,
}

impl TicketAccount {
    pub const SPACE: usize = 8 +  // ticket_number
                            32 + // owner
                            32 + // lottery
                            1 +  // is_winner
                            1 +  // claimed
                            8 +  // prize_amount
                            1;   // bump
}