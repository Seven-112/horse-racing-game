use anchor_lang::prelude::*;

use crate::state::*;
use crate::utils::*;
use crate::errors::*;

#[event]
pub struct RaceEvent {
    pub winners: [Pubkey; 10],
    pub winner_count: u8
}

#[derive(Accounts)]
pub struct StartRace<'info> {

    #[account(mut, signer)]
    pub operator: AccountInfo<'info>,

    #[account(mut)]
    pub race_result: Account<'info, RaceResult>,

    #[account(
        mut,
        constraint = nft_list.owner == program_id
    )]
    pub nft_list: AccountInfo<'info>,    
    
    #[account(
        mut,
        seeds = [
            b"operator_list".as_ref()
        ],
        bump = operator_list.bump
    )]
    pub operator_list: Account<'info, OperatorWhiteList>,

    pub sol_feed_account: AccountInfo<'info>,

    pub btc_feed_account: AccountInfo<'info>,

    pub clock: Sysvar<'info, Clock>
}


pub fn process(
    ctx: Context<StartRace>
) -> ProgramResult {

    let operator_list = &mut ctx.accounts.operator_list;
    let mut flag: u8 = 0;
    for item in operator_list.operator_array.iter().enumerate() {
        let (_, op): (usize, &Pubkey) = item;
        if *op == ctx.accounts.operator.key() {
            flag = 1;
            break;
        }
    }

    if flag != 1 {
        msg!("You are not operator to start race");
        return Err(ErrorCode::InvalidOperator.into());
    }

    let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
    let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;

    //let sol_price: Option<u128> = Some(10);
    //let btc_price: Option<u128> = Some(20);

    let mut random_value: u128 = 0;
    if let Some(sol_price) = sol_price {
        random_value += sol_price;
        msg!("Sol Price is {}", sol_price);
    } else {
        msg!("No current Sol price");
    }

    if let Some(btc_price) = btc_price {
        random_value += btc_price;
        msg!("BTC Price is {}", btc_price);
    } else {
        msg!("No current BTC price");
    }
    random_value += ctx.accounts.clock.unix_timestamp as u128;
    
    prize_winner(
        (random_value % 20) as u8, 
        ctx.accounts
    )?;

    Ok(())
}

pub fn prize_winner<'info> (
    random_value: u8,
    accounts: &mut StartRace<'info>
) -> ProgramResult {

    let mut nft_list_data = accounts.nft_list.data.borrow_mut();
    let count: u16 = u16::try_from_slice(&nft_list_data[0..2])?;

    let mut score_arr: Vec<Score> = vec![];
    let cnt: usize = count as usize;

    for i in 0..cnt {
        let start = 2 + NFT_ITEM_SIZE * i;
        let t_passion = nft_list_data[start + 32];
        let t_stamina = nft_list_data[start + 33];
        score_arr.push(Score {
            score: (t_passion + t_stamina + random_value) as u16,
            nft_id: i as u16
        });
    }
    for i in 0..cnt {
        for j in i+1..cnt {
            if score_arr[i].score < score_arr[j].score {
                let temp = score_arr[i];
                score_arr[i] = score_arr[j];
                score_arr[j] = temp;
            }
        }
    }

    let prize: [u8; 10] = [70, 20, 3, 1, 1, 1, 1, 1, 1, 1];
    msg!("prize = {:?}", prize);

    let winner_count = min(count, 10) as usize;
    for i in 0..winner_count {
        let p: u32 = prize[i] as u32;
        let idx = score_arr[i].nft_id as usize;
        let start = 2 + NFT_ITEM_SIZE*idx;
        p.serialize(&mut &mut nft_list_data[start+34..start+38])?;

        let nft_key = Pubkey::try_from_slice(&nft_list_data[start..start+32])?;
        accounts.race_result.winners[i] = nft_key;
    }
    accounts.race_result.winner_cnt = winner_count as u8;

    emit!(RaceEvent {
        winners: accounts.race_result.winners,
        winner_count: accounts.race_result.winner_cnt
    });

    Ok(())
}