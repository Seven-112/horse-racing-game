use anchor_lang::prelude::*;
use crate::state::*;

pub struct GetAward<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn process(
    ctx: Context<GetAward>
) -> ProgramResult {
    Ok(())
}