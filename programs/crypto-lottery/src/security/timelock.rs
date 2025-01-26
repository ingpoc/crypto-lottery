use anchor_lang::prelude::*;
use crate::models::error::LotteryError;

#[account]
pub struct Timelock {
    pub authority: Pubkey,
    pub delay: i64,
    pub min_delay: i64,
    pub max_delay: i64,
    pub queued_tx: Option<QueuedTransaction>,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct QueuedTransaction {
    pub instruction_data: Vec<u8>,
    pub queued_at: i64,
    pub eta: i64,
}

impl Timelock {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        8 + // delay
        8 + // min_delay
        8 + // max_delay
        (1 + (4 + 256 + 8 + 8)) + // queued_tx option
        1; // bump

    pub fn queue_transaction(
        &mut self,
        instruction_data: Vec<u8>,
        eta: i64,
        clock: &Clock,
    ) -> Result<()> {
        require!(
            instruction_data.len() <= 256,
            LotteryError::InvalidState
        );
        
        require!(
            eta >= clock.unix_timestamp + self.delay &&
            eta <= clock.unix_timestamp + self.max_delay,
            LotteryError::InvalidState
        );

        self.queued_tx = Some(QueuedTransaction {
            instruction_data,
            queued_at: clock.unix_timestamp,
            eta,
        });

        Ok(())
    }

    pub fn execute_transaction(&mut self, clock: &Clock) -> Result<Vec<u8>> {
        let tx = self.queued_tx.as_ref()
            .ok_or(LotteryError::InvalidState)?;
        
        require!(
            clock.unix_timestamp >= tx.eta,
            LotteryError::InvalidState
        );

        let instruction_data = tx.instruction_data.clone();
        self.queued_tx = None;
        
        Ok(instruction_data)
    }

    pub fn cancel_transaction(&mut self) -> Result<()> {
        require!(
            self.queued_tx.is_some(),
            LotteryError::InvalidState
        );
        
        self.queued_tx = None;
        Ok(())
    }
}