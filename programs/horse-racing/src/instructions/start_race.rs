use anchor_lang::prelude::*;
use create::state::*;

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
        return Err(ProgramError::InvalidAccountData);
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

    let nft_count = get_nft_count(ctx.accounts.nft_list.clone())?;
    for i in 0..nft_count {

    }
    let winner_idx = (random_value % nft_count as u128) as u64;
    msg!("winner is {} th horse!", winner_idx);

    // getting Pubkey of winner
    /*let winner_pk = prize_winner((winner_idx % 20) as u8, ctx.accounts.nft_list.clone())?;
    ctx.accounts.race_result.winner = winner_pk;*/
    Ok(())
}

pub fn prize_winner<'info> (
    random_value: u8,
    nft_list: AccountInfo<'info>
) -> ProgramResult {

    let mut nft_list_data = nft_list.data.borrow_mut();
    let count: u16 = u16::try_from_slice(&nft_list_data[0..2])?;

    let item_size: usize = 32 + 1 + 1 + 4;

    let mut t_passion: u8 = 0;
    let mut t_stamina: u8 = 0;

    let mut score_arr: [Score; 1000] = [Score { nft_id: 0, score: 0 }; 1000];

    let cnt: usize = count as usize;

    for i in 0..cnt {
        let start = 2 + item_size * i;
        t_passion = nft_list_data[start + 32];
        t_stamina = nft_list_data[start + 33];
        score_arr[i].score = (t_passion + t_stamina + random_value) as u16;
        score_arr[i].nft_id = i as u16;
    }

    for i in 0..cnt {
        for j in i+1..cnt {
            if score_arr[i].score < score_arr[j].score {
                let t = score_arr[i];
                score_arr[i] = score_arr[j];
                score_arr[j] = t;
            }
        }
    }

    let prize = [0.7, 0.2, 0.03, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01, 0.01];

    for i in 0..min(cnt as u16, 10) {
        let p: u32 = (prize[i] * 100.0) as u32;
        let idx = score_arr[i].nft_id as usize;
        let start = 2 + item_size*idx;
        p.serialize(&mut &mut nft_list_data[start+34..start+38]);
    }

    Ok(())
}