use anchor_lang::prelude::*;
use anchor_spl::token::{ Token };

use crate::state::*;
use crate::constants::SEED_PDA;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = owner,
        seeds = [SEED_PDA, owner.key().as_ref()],
        bump,
        space = ConfigAccount::LEN
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler_init<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    let config_account = &mut ctx.accounts.config_account;
    config_account.config_bump = *ctx.bumps.get("config_account").unwrap();
    Ok(())
}

impl<'info> Initialize<'info> {
    // do something later
}