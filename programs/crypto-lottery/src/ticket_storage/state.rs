use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug)]
pub struct TicketMetadata {
    pub owner: Pubkey,
    pub purchase_time: i64,
    pub is_winner: bool,
    pub prize_claimed: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Ticket {
    pub number: u32,
    pub metadata: TicketMetadata,
}