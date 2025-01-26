use anchor_lang::prelude::*;
use crate::models::{error::LotteryError, constants::MAX_TICKET_COUNT};

#[account]
pub struct TicketBucket {
    pub tickets: Vec<Ticket>,
    pub next_bucket: Option<Pubkey>,
    pub total_tickets: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct Ticket {
    pub owner: Pubkey,
    pub number: u64,
    pub purchase_time: i64,
}

impl TicketBucket {
    pub const SIZE: usize = 8 + // discriminator
        4 + (MAX_TICKET_COUNT * (32 + 8 + 8)) + // tickets vec
        (1 + 32) + // next_bucket option
        8 + // total_tickets
        1; // bump

    pub fn add_ticket(
        &mut self,
        owner: Pubkey,
        number: u64,
        purchase_time: i64,
    ) -> Result<()> {
        require!(
            self.tickets.len() < MAX_TICKET_COUNT,
            LotteryError::InvalidState
        );

        self.tickets.push(Ticket {
            owner,
            number,
            purchase_time,
        });
        
        self.total_tickets = self.total_tickets.checked_add(1)
            .ok_or(LotteryError::ArithmeticOverflow)?;
            
        Ok(())
    }

    pub fn get_ticket(&self, number: u64) -> Option<&Ticket> {
        self.tickets.binary_search_by_key(&number, |ticket| ticket.number)
            .ok()
            .map(|idx| &self.tickets[idx])
    }

    pub fn get_tickets_by_owner(&self, owner: &Pubkey) -> Vec<&Ticket> {
        self.tickets.iter()
            .filter(|ticket| ticket.owner == *owner)
            .collect()
    }

    pub fn validate_ticket(&self, owner: &Pubkey, number: u64) -> Result<()> {
        let ticket = self.get_ticket(number)
            .ok_or(LotteryError::InvalidTicket)?;
            
        require!(
            ticket.owner == *owner,
            LotteryError::UnauthorizedAccess
        );
        
        Ok(())
    }
}