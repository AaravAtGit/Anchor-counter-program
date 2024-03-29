use anchor_lang::prelude::*;

declare_id!("5o7kE3V4FfkQT5pTZW8PkkzYrMdUTGnSqqp3KZYsmiYk");

#[program]
pub mod counter_program {
    use super::*;

    // Create method to initialize the counter account
    pub fn create(ctx: Context<Create>) -> Result<()> {
        // Set the authority of the counter account to the authority key
        ctx.accounts.counter.authority = ctx.accounts.authority.key();
        // Set the count of the counter account to 0
        ctx.accounts.counter.count = 0;
        Ok(())
    }

    // Increment method to increase the count by 1
    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        // Increment the count by 1
        ctx.accounts.counter.count += 1;
        Ok(())
    }

    // Decrement method to decrease the count by 1
    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        // Decrement the count by 1
        ctx.accounts.counter.count -= 1;
        Ok(())
    }
}

// define the accounts required for the Create method
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = authority, space = 8 + 8 + 32)]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// define the accounts required for the Increment method
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

//define the accounts required for the Decrement method
#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut, has_one = authority)]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

//represent the counter account
#[account]
pub struct Counter {
    pub authority: Pubkey,
    pub count: u64,
}
