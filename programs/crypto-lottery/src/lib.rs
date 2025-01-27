use anchor_lang::prelude::*;
use anchor_spl::{
    token::Token,
    associated_token::AssociatedToken,
};
use models::state::*;

pub mod instructions;
pub mod models;
pub mod security;
pub mod ticket_storage;

declare_id!("F5q6vpnQdKCJqKZF19bQibPhvLbMMWMso1RCtwLH3NEJ");

#[program]
pub mod crypto_lottery {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize_handler(ctx)
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()> {
        instructions::buy_ticket_handler(ctx, amount)
    }

    pub fn select_winner(ctx: Context<SelectWinner>) -> Result<()> {
        instructions::select_winner_handler(ctx)
    }

    pub fn claim_prize(ctx: Context<ClaimPrize>) -> Result<()> {
        instructions::claim_prize_handler(ctx)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + ProgramConfig::SPACE)]
    pub config: Account<'info, ProgramConfig>,
    
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub lottery: Account<'info, LotteryAccount>,
    #[account(
        mut,
        seeds = [b"ticket"],
        bump,
    )]
    pub ticket_account: Account<'info, TicketAccount>,
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(
        mut,
        constraint = player_token_account.key() == player.key()
    )]
    /// CHECK: This is safe because we check the owner constraint
    pub player_token_account: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = lottery_token_account.key() == lottery.key()
    )]
    /// CHECK: This is safe because we check the owner constraint
    pub lottery_token_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SelectWinner<'info> {
    #[account(mut)]
    pub lottery: Account<'info, LotteryAccount>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ClaimPrize<'info> {
    #[account(mut)]
    pub lottery: Account<'info, LotteryAccount>,
    #[account(mut)]
    pub winner: Signer<'info>,
    #[account(
        mut,
        constraint = winner_token_account.key() == winner.key()
    )]
    /// CHECK: This is safe because we check the owner constraint
    pub winner_token_account: UncheckedAccount<'info>,
    #[account(
        mut,
        constraint = lottery_token_account.key() == lottery.key()
    )]
    /// CHECK: This is safe because we check the owner constraint
    pub lottery_token_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}