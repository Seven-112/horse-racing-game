use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::*;

#[event]
struct DelOperatorEvent {
    admin: Pubkey,
    removed_operator: Pubkey
}

#[derive(Accounts)]
pub struct DelOperator<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,
    
    pub operator: AccountInfo<'info>,

    #[account(mut)]
    pub operator_list: Account<'info, OperatorWhiteList>
}

pub fn process(
    ctx: Context<DelOperator>
) -> ProgramResult {
    let operator_list = &mut ctx.accounts.operator_list;
    let cnt: usize = operator_list.operator_cnt as usize;

    if cnt == 1 {
        return Err(ErrorCode::OperatorNotEnough.into());
    }

    if *ctx.accounts.admin.key != operator_list.operator_array[0] {
        return Err(ErrorCode::UnknownAdmin.into());
    }

    let mut op_flag: u8 = 0;
    for i in 0..cnt as usize {
        if operator_list.operator_array[i] == *ctx.accounts.operator.key {
            if i != cnt - 1 {
                for j in i..cnt-1 {
                    operator_list.operator_array[j] = operator_list.operator_array[j+1];
                }
            }
            op_flag = 1;
            break;
        }
    }
    operator_list.operator_cnt -= 1;

    if op_flag == 0 {
        return Err(ErrorCode::OperatorNotFound.into());
    }

    emit!(DelOperatorEvent{
        admin: ctx.accounts.admin.key(),
        removed_operator: ctx.accounts.operator.key()
    });

    Ok(())
}