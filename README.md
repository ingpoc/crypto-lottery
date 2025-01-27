# Solana Lottery Program

A decentralized lottery system built on Solana blockchain featuring daily, weekly, and monthly draws.

## Architecture

```
crypto-lottery-program/
├── programs/
│   └── crypto-lottery/
│       ├── src/
│       │   ├── instructions/     # Instruction handlers
│       │   │   ├── initialize.rs
│       │   │   ├── buy_ticket.rs
│       │   │   ├── batch_process.rs
│       │   │   └── cleanup.rs
│       │   ├── models/          # Data structures
│       │   │   ├── state.rs
│       │   │   ├── error.rs
│       │   │   └── events.rs
│       │   ├── security/        # Security components
│       │   │   ├── random.rs
│       │   │   └── timelock.rs
│       │   ├── ticket_storage/  # Ticket management
│       │   │   └── mod.rs
│       │   └── lib.rs          # Program entry point
│       └── Cargo.toml
├── tests/                      # Test suite
└── migrations/                 # Deployment scripts

```

## Features

- Multiple lottery periods (daily/weekly/monthly)
- Secure random number generation
- Timelock mechanism for admin operations
- Batch processing for scalability
- Automated winner selection

## Security Features

- Progressive hash reveal mechanism
- Timelock for admin operations
- Secure RNG using multiple blockhashes
- Access control validation
- Transaction timeout protection

## API Documentation

### Program Instructions

1. Initialize Program
```rust
pub fn initialize(ctx: Context<Initialize>) -> Result<()>
```
- Creates lottery configuration
- Initializes ticket storage
- Sets up timelock mechanism

2. Buy Ticket
```rust
pub fn buy_ticket(ctx: Context<BuyTicket>, amount: u64) -> Result<()>
```
- Parameters:
  - amount: Number of tickets to purchase
- Validates token accounts
- Processes token transfer
- Assigns ticket numbers

3. Process Batch
```rust
pub fn process_batch(ctx: Context<BatchProcess>) -> Result<()>
```
- Processes tickets in batches
- Updates lottery state
- Emits batch events

4. Select Winner
```rust
pub fn select_winner(ctx: Context<SelectWinner>) -> Result<()>
```
- Generates winning number
- Updates lottery state
- Distributes prizes

### Account Structures

1. LotteryAccount
```rust
pub struct LotteryAccount {
    pub lottery_type: LotteryType,
    pub status: LotteryStatus,
    pub start_time: i64,
    pub end_time: i64,
    pub ticket_price: u64,
    pub total_tickets: u64,
    pub prize_pool: u64,
    pub winning_number: Option<u64>,
    pub bump: u8,
}
```

2. TicketBucket
```rust
pub struct TicketBucket {
    pub tickets: Vec<Ticket>,
    pub next_bucket: Option<Pubkey>,
    pub total_tickets: u64,
    pub bump: u8,
}
```

### Events

1. LotteryCreated
```rust
pub struct LotteryCreated {
    pub lottery: Pubkey,
    pub lottery_type: String,
    pub start_time: i64,
    pub end_time: i64,
    pub ticket_price: u64,
}
```

2. TicketPurchased
```rust
pub struct TicketPurchased {
    pub lottery: Pubkey,
    pub buyer: Pubkey,
    pub amount: u64,
    pub timestamp: i64,
}
```

## Getting Started

1. Install Dependencies
```bash
npm install
```

2. Build Program
```bash
anchor build
```

3. Deploy
```bash
solana program deploy target/deploy/crypto_lottery.so
```

## Testing

Run test suite:
```bash
anchor test
```

## Security Considerations

1. Admin Operations
- Use timelock for sensitive operations
- Multi-signature support for critical functions

2. Random Number Generation
- Multiple blockhash sources
- Progressive reveal mechanism

3. Token Handling
- SPL token integration
- Escrow-based transfers

## License

MIT