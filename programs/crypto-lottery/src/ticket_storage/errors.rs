use anchor_lang::prelude::*;

#[error_code]
pub enum TicketError {
    #[msg("Ticket bucket is full")]
    BucketFull,
    #[msg("Invalid ticket number")]
    InvalidTicketNumber,
    #[msg("Ticket not found")]
    TicketNotFound,
    #[msg("Invalid claimer")]
    InvalidClaimer,
    #[msg("Ticket is not a winner")]
    NotAWinner,
    #[msg("Prize already claimed")]
    PrizeAlreadyClaimed,
}