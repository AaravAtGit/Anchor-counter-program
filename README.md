# Anchor Counter Program

This repository contains a basic Solana program developed using the Anchor framework for managing a counter. The counter program provides methods to create, increment, and decrement a counter account.

## Code Overview

### Program Declaration

```rust
use anchor_lang::prelude::*;

declare_id!("5o7kE3V4FfkQT5pTZW8PkkzYrMdUTGnSqqp3KZYsmiYk");

#[program]
pub mod counter_program {
    use super::*;

    // Methods for the counter program
    pub fn create(ctx: Context<Create>) -> Result<()> {
        // Logic for creating a counter account
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Logic for incrementing the counter
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        // Logic for decrementing the counter
        Ok(())
    }
}
```

- The `counter_program` module declares methods (`create`, `increment`, `decrement`) for the counter program.

### Create Method

```rust
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn create(ctx: Context<Create>) -> Result<()> {
    // Logic for initializing the counter account
    Ok(())
}
```

- The `Create` struct defines the accounts required for the `create` method. It initializes a counter account with space for authority and count fields.
- The `create` method initializes the counter account, setting the authority and count fields.

### Increment Method

```rust
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

pub fn increment(ctx: Context<Increment>) -> Result<()> {
    // Logic for incrementing the counter
    Ok(())
}
```

- The `Increment` struct defines the accounts required for the `increment` method. It ensures the presence of the authority.
- The `increment` method increments the count field of the counter account.

### Decrement Method

```rust
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
    // Logic for decrementing the counter
    Ok(())
}
```

- The `Decrement` struct defines the accounts required for the `decrement` method. It ensures the presence of the authority.
- The `decrement` method decrements the count field of the counter account.

### Counter Account

```rust
#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
```

- The `Counter` struct represents the state of the counter account, containing the authority and count fields.

## Usage

1. Clone the repository.
2. Ensure you have Rust and Solana tools installed.
3. Run `anchor build` to build the program.
4. Deploy the program using Solana CLI or other deployment methods.

Feel free to explore, modify, and test the counter program according to your requirements. If you encounter any issues or have questions, refer to the provided documentation or seek assistance in the comments. Happy coding!
