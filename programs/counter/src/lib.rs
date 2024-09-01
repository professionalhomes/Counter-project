use anchor_lang::prelude::*;

declare_id!("C93fyDjEmyAfr9nwDeWMVCeWVVx8fjySxnshSA9VY4KG");

#[programm]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count: {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous couter: {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count: {}", counter.count);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub user: Singer<'info>,
    #[account(
        init,
        payer = user,
        space = 8 + 8
    )]
    pub  counter: Account<'info, Counter>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}
//hello
#[account]
pub struct Counter {
    pub count: u64,
}