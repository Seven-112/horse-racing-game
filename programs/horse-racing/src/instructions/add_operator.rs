use anchor_lang::prelude::*;
use crate::state::*;

#[event]
struct AddOperatorEvent {
    adder: Pubkey,
    new_operator: Pubkey
}

#[derive(Accounts)]
pub struct AddOperator<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    
    pub operator: AccountInfo<'info>,

    #[account(mut)]
    pub operator_list: Account<'info, OperatorWhiteList>
}

pub fn process(
    ctx: Context<AddOperator>
) -> ProgramResult {
    let operator_list = &mut ctx.accounts.operator_list;
    let cnt: usize = operator_list.operator_cnt as usize;

    operator_list.operator_array[cnt] = ctx.accounts.operator.key();
    operator_list.operator_cnt += 1;

    emit!(AddOperatorEvent{
        adder: ctx.accounts.admin.key(),
        new_operator: ctx.accounts.operator.key()
    });

    Ok(())
}