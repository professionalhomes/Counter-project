use anchor_lang::prelude::*;

declare_id!("9EDzaCdJMbBpbXgy8Q9owegoNN2RrFGFetfwcgHNuApn");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
