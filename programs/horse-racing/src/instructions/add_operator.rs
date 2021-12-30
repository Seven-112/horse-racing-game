use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

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

    if cnt == 10 {
        return Err(ErrorCode::OperatorOverflow.into());
    }

    if *ctx.accounts.admin.key != operator_list.operator_array[0] {
        return Err(ErrorCode::UnknownAdmin.into());
    }

    let mut op_flag: u8 = 0;
    for i in 0..cnt as usize {
        if operator_list.operator_array[i] == ctx.accounts.operator.key() {
            op_flag = 1;
            break;
        }
    }

    if op_flag == 1 {
        return Err(ErrorCode::OperatorDuplicated.into());
    }

    operator_list.operator_array[cnt] = ctx.accounts.operator.key();
    operator_list.operator_cnt += 1;

    emit!(AddOperatorEvent{
        adder: ctx.accounts.admin.key(),
        new_operator: ctx.accounts.operator.key()
    });

    Ok(())
}