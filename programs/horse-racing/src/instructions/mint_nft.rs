use anchor_lang::prelude::*;
use create::state::*;

#[derive(Accounts)]
#[instruction(metadata_bump : u8)]
pub struct MintNFT<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        constraint = nft_list.owner == program_id
    )]
    pub nft_list: AccountInfo<'info>,    

    #[account(
        init,
        seeds = [(*mint.key).as_ref()],
        bump = metadata_bump,
        payer = owner,
        space = 8 + UPGRADABLE_METASIZE
    )]
    pub upgradable_metadata: Account<'info, UpgradableMetadata>,

    #[account(owner = spl_token::id())]
    pub mint: AccountInfo<'info>,

    #[account(
        constraint = token_account.mint == *mint.key,
        constraint = *token_account.to_account_info().owner == token::Token::id()
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub sol_feed_account: AccountInfo<'info>,

    pub btc_feed_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>
}

pub fn process(
    ctx: Context<MintNFT>,
    metadata_bump: u8
) -> ProgramResult {

    let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;
    upgradable_metadata.bump = metadata_bump;
    upgradable_metadata.passion = 0;
    upgradable_metadata.stamina = 0;

    let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
    let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;

    //let sol_price: Option<u128> = Some(10);
    //let btc_price: Option<u128> = Some(20);

    msg!("after getprice");
    if let Some(sol_price) = sol_price {
        let rand_from_sol = sol_price + ctx.accounts.clock.unix_timestamp as u128;
        upgradable_metadata.passion = (rand_from_sol % 10) as u8 + MIN_PASSION;
        msg!("Sol Price is {}", sol_price);
    } else {
        upgradable_metadata.passion = (ctx.accounts.clock.unix_timestamp % 10) as u8 + MIN_PASSION;
        msg!("No current Sol price");
    }

    if let Some(btc_price) = btc_price {
        let rand_from_sol = btc_price + ctx.accounts.clock.unix_timestamp as u128;
        upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + MIN_STAMINA;
        msg!("BTC Price is {}", btc_price);
    } else {
        upgradable_metadata.stamina = (ctx.accounts.clock.unix_timestamp % 10) as u8 + MIN_STAMINA;
        msg!("No current BTC price");
    }

    add_nft(
        ctx.accounts.mint.key(), 
        ctx.accounts.nft_list.clone(),
        &upgradable_metadata
    )?;

    Ok(())
}

pub fn add_nft<'info> (
    nft_mint: Pubkey,
    nft_list: AccountInfo<'info>,
    ex_metadata: &UpgradableMetadata
) -> ProgramResult {
    let mut count = get_nft_count(nft_list.clone())?;
    let mut nft_list_data = nft_list.data.borrow_mut();

    let item_size: usize = 32 + 1 + 1 + 4;
    let start: usize = (2 + item_size * 32) as usize;
    nft_mint.serialize(&mut &mut nft_list_data[start..start + 32])?;
    nft_list_data[start + 32] = ex_metadata.passion;
    nft_list_data[start + 33] = ex_metadata.stamina;

    let pending_reward: u32 = 0;
    pending_reward.serialize(&mut &mut nft_list_data[start+34..start+38])?;

    count += 1;
    let data = count.to_le_bytes();
    for i in 0..2 {
        nft_list_data[i] = data[i];
    }

    Ok(())
}