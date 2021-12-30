use anchor_lang::prelude::*;

use crate::state::*;
use crate::utils::*;
use crate::errors::*;

#[event]
pub struct TransferRoleEvent {
    pub admin: Pubkey,
    pub new_admin: Pubkey
}

#[derive(Accounts)]
pub struct TransferRole<'info> {
    #[account(signer)]
    pub admin: AccountInfo<'info>,

    pub new_admin: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            b"operator_list".as_ref()
        ],
        bump = operator_list.bump
    )]
    pub operator_list: Account<'info, OperatorWhiteList>,
}

pub fn process(
    ctx: Context<TransferRole>
) -> ProgramResult {
    let operator_list = &mut ctx.accounts.operator_list;
    let cnt: usize = operator_list.operator_cnt as usize;
    let mut op_flag: u8 = 0;
    for i in 0..cnt as usize {
        if operator_list.operator_array[i] == *ctx.accounts.new_admin.key {
            if i != cnt - 1 {
                for j in i..cnt-1 {
                    operator_list.operator_array[j] = operator_list.operator_array[j+1];
                }
            }
            op_flag = 1;
            break;
        }
    }
    
    if op_flag == 0 {
        return Err(ErrorCode::TransferRoleToNormalUser.into());
    }

    // make super admin
    operator_list.operator_array[0] = *ctx.accounts.new_admin.key;
    operator_list.operator_array[cnt-1] = *ctx.accounts.admin.key;

    emit!(TransferRoleEvent {
        admin: *ctx.accounts.admin.key,
        new_admin: *ctx.accounts.new_admin.key,
    });

    Ok(())
}
