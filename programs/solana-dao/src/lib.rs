use anchor_lang::prelude::*;

declare_id!("3s54kTesUM3ii6XFBxab9oEyfJVX4qhuNzcYf2qd98AP");

#[program]
pub mod solana_dao {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
