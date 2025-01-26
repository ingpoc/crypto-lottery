use anchor_lang::prelude::*;
use std::convert::TryInto;

pub fn generate_ticket_number(
    timestamp: i64,
    user_pubkey: &Pubkey,
    counter: u32,
) -> Result<u32> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(&timestamp.to_le_bytes());
    hasher.update(user_pubkey.as_ref());
    hasher.update(&counter.to_le_bytes());
    
    let hash = hasher.finalize();
    let bytes: [u8; 4] = hash.as_bytes()[..4].try_into().unwrap();
    let number = u32::from_le_bytes(bytes);
    
    // Ensure number is between 100000 and 999999
    Ok(100_000 + (number % 900_000))
}