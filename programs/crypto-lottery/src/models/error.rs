use anchor_lang::prelude::*;

#[error_code]
pub enum LotteryError {
    #[msg("Invalid lottery state")]
    InvalidState,
    
    #[msg("Lottery not found")]
    LotteryNotFound,
    
    #[msg("Insufficient ticket funds")]
    InsufficientFunds,
    
    #[msg("Invalid ticket number")]
    InvalidTicket,
    
    #[msg("Lottery round ended")]
    LotteryEnded,
    
    #[msg("Unauthorized access")]
    UnauthorizedAccess,
    
    #[msg("No prize to claim")]
    NoPrizeToClaim,
    
    #[msg("Invalid prize amount")]
    InvalidPrizeAmount,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,
}