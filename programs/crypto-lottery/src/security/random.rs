use anchor_lang::prelude::*;
use solana_program::keccak::hashv;

pub struct RandomNumberGenerator {
    recent_blockhashes: Vec<[u8; 32]>,
    lottery_pubkey: Pubkey,
    timestamp: i64,
    salt: [u8; 32],
}

impl RandomNumberGenerator {
    pub fn new(lottery_pubkey: Pubkey, timestamp: i64, salt: [u8; 32]) -> Self {
        Self {
            recent_blockhashes: Vec::new(),
            lottery_pubkey,
            timestamp,
            salt,
        }
    }

    pub fn add_blockhash(&mut self, blockhash: [u8; 32]) {
        self.recent_blockhashes.push(blockhash);
    }

    pub fn generate(&self) -> [u8; 32] {
        let mut data = Vec::with_capacity(32 + 8 + self.recent_blockhashes.len() * 32 + 32);
        data.extend(self.lottery_pubkey.to_bytes());
        data.extend(self.timestamp.to_le_bytes());
        
        for hash in &self.recent_blockhashes {
            data.extend(hash);
        }
        
        data.extend(self.salt);
        hashv(&[&data]).to_bytes()
    }

    pub fn generate_in_range(&self, min: u64, max: u64) -> u64 {
        let random_bytes = self.generate();
        let random_u64 = u64::from_le_bytes(random_bytes[0..8].try_into().unwrap());
        min + (random_u64 % (max - min + 1))
    }
}