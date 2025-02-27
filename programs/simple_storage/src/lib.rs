use anchor_lang::prelude::*;

declare_id!("NgoV63pHvHu6AE8oQ4XzRMt4A4DGXmuEF7SY1RxQf9W");

#[program]
pub mod simple_storage {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        storage.data = String::from("Hello, Solana!");
        msg!("Account initialized with: {}", storage.data);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, new_data: String) -> Result<()> {
        let storage = &mut ctx.accounts.storage;
        msg!("Updating storage from '{}' to '{}'", storage.data, new_data);
        storage.data = new_data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 100)]
    pub storage: Account<'info, Storage>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub storage: Account<'info, Storage>,
}

#[account]
pub struct Storage {
    pub data: String,
}
