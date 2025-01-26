pub const TREASURY_FEE: u16 = 250; // 2.5%
pub const DAILY_LOTTERY_PRICE: u64 = 1_000_000; // 1 USDC (6 decimals)
pub const WEEKLY_LOTTERY_PRICE: u64 = 5_000_000; // 5 USDC
pub const MONTHLY_LOTTERY_PRICE: u64 = 10_000_000; // 10 USDC
pub const MAX_TICKET_COUNT: usize = 100_000; // Max tickets per lottery
pub const TICKET_SEED: &[u8] = b"ticket";

// Prize distribution percentages (basis points)
pub const MATCH_6_PRIZE_SHARE: u16 = 6000; // 60%
pub const MATCH_5_PRIZE_SHARE: u16 = 2500; // 25%
pub const MATCH_4_PRIZE_SHARE: u16 = 1000; // 10%
pub const MATCH_3_PRIZE_SHARE: u16 = 500;  // 5%