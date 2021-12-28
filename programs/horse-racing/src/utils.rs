use anchor_lang::prelude::*;
use solana_program::{
    program_error::ProgramError,
    program::{invoke},
}

pub fn get_nft_count<'a> (
    nft_list: AccountInfo<'a>
) -> Result<u16, ProgramError> {
    let nft_list_data = nft_list.data.borrow();
    let count: u16 = u16::try_from_slice(&nft_list_data[0..2])?;
    Ok(count)
}

// transfer sol
fn sol_transfer<'a>(
    source: AccountInfo<'a>,
    destination: AccountInfo<'a>,
    system_program: AccountInfo<'a>,
    amount: u64,
) -> Result<(), ProgramError> {
    let ix = solana_program::system_instruction::transfer(source.key, destination.key, amount);
    invoke(&ix, &[source, destination, system_program])
}

fn min(x: u16, y: u16) -> usize {
    if (x < y) {
        return x as usize;
    }
    y as usize
}