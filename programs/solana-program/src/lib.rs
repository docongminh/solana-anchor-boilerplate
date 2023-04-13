use anchor_lang::prelude::*;

pub mod instructions;
pub mod constants;
pub mod state;
pub mod error;

use crate::instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_program {
    use super::*;

    pub fn initialize<'info>(_ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        Ok(())
    }
}