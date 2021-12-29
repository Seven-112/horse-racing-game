use anchor_lang::prelude::*;
use crate::state::*;

#[event]
struct InitializeEvent {
    pub initializer: Pubkey,
}

#[derive(Accounts)]
#[instruction(operator_list_bump: u8, race_bump: u8)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        constraint = nft_list_account.owner == program_id
    )]
    pub nft_list_account: AccountInfo<'info>,

    #[account(
        init,
        seeds = [
            b"operator_list".as_ref()
        ],
        bump = operator_list_bump,
        payer = admin,
        space = 8 + OPERATOR_LIST_SIZE
    )]
    pub operator_list: Account<'info, OperatorWhiteList>,

    #[account(
        init,
        seeds = [
            b"race_result".as_ref()
        ],
        bump = race_bump,
        payer = admin,
        space = 8 + RACE_RESULT_SIZE
    )]
    pub race_result: Account<'info, RaceResult>,

    pub system_program: Program<'info, System>,
}

pub fn process(
    ctx: Context<Initialize>,
    operator_list_bump: u8,
    race_bump: u8
) -> ProgramResult {

    let operator_list = &mut ctx.accounts.operator_list;
    operator_list.operator_array[0] = ctx.accounts.admin.key();
    operator_list.operator_cnt = 1;
    operator_list.bump = operator_list_bump;

    let race_result = &mut ctx.accounts.race_result;
    race_result.bump = race_bump;

    emit!(InitializeEvent {
        initializer: ctx.accounts.admin.key()
    });
    
    Ok(())
}