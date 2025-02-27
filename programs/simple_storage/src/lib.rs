use anchor_lang::prelude::*;

declare_id!("NgoV63pHvHu6AE8oQ4XzRMt4A4DGXmuEF7SY1RxQf9W");

#[program]
pub mod simple_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
