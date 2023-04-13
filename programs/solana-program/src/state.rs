use anchor_lang::prelude::*;

#[account]
pub struct ConfigAccount {
    pub config_bump: u8,
}

impl ConfigAccount {
    pub const LEN: usize = 8 + 1 * 1; //u8
}