use anchor_lang::prelude::*;

#[event]
pub struct TicketPurchased {
    pub lottery: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct WinnerSelected {
    pub lottery: Pubkey,
    pub winner: Pubkey,
    pub prize_amount: u64,
    pub winning_ticket: u64,
}