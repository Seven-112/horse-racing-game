use anchor_lang::prelude::*;
use create::state::*;

#[derive(Accounts)]
pub struct AddOperator<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    
    pub operator: AccountInfo<'info>,

    #[account(mut)]
    pub operator_list: Account<'info, OperatorWhiteList>,
}

pub fn process(
    ctx: Context<AddOperator>
) -> ProgramResult {
    let operator_list = &mut ctx.accounts.operator_list;
    let cnt: usize = operator_list.operator_cnt as usize;

    operator_list.operator_array[cnt] = ctx.accounts.operator.key();
    operator_list.operator_cnt += 1;
    Ok(())
}