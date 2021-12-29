use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount};

use crate::state::*;
use crate::utils::*;
use crate::errors::*;

#[event]
pub struct UpgradeEvent {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub old_passion: u8,
    pub old_stamina: u8,
    pub passion: u8,
    pub stamina: u8,
}

#[derive(Accounts)]
pub struct UpgradeNFT<'info> {
    #[account(
        mut,
        constraint = admin.key() == operator_list.operator_array[0]
    )]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,    
    
    #[account(
        mut,
        constraint = nft_list.owner == program_id
    )]
    pub nft_list: AccountInfo<'info>,   

    #[account(
        mut,
        seeds = [(*mint.key).as_ref()],
        bump = upgradable_metadata.bump
    )]
    pub upgradable_metadata: Account<'info, UpgradableMetadata>,

    #[account(owner = token::Token::id())]
    pub mint: AccountInfo<'info>,

    #[account(
        constraint = token_account.mint == *mint.key,
        constraint = *token_account.to_account_info().owner == token::Token::id()
    )]
    pub token_account: Account<'info, TokenAccount>,

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

    pub system_program: Program<'info, System>,

    pub clock: Sysvar<'info, Clock>
}

pub fn process(
    ctx: Context<UpgradeNFT>,
    nft_id: u16
) -> ProgramResult {

    let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;

    let sol_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.sol_feed_account)?;
    let btc_price = chainlink::get_price(&chainlink::id(), &ctx.accounts.btc_feed_account)?;
    
    //let sol_price: Option<u128> = Some(10);
    //let btc_price: Option<u128> = Some(20);
    
    let old_passion = upgradable_metadata.passion;
    let old_stamina = upgradable_metadata.stamina;

    if let Some(sol_price) = sol_price {
        let rand_from_sol = sol_price + ctx.accounts.clock.unix_timestamp as u128;
        upgradable_metadata.passion = (rand_from_sol % 10) as u8 + upgradable_metadata.passion;
        msg!("Sol Price is {}", sol_price);
    } else {
        upgradable_metadata.passion = (ctx.accounts.clock.unix_timestamp % 10) as u8 + upgradable_metadata.passion;
        msg!("No current Sol price");
    }

    if let Some(btc_price) = btc_price {
        let rand_from_sol = btc_price + ctx.accounts.clock.unix_timestamp as u128;
        upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + upgradable_metadata.stamina;
        msg!("BTC Price is {}", btc_price);
    } else {
        upgradable_metadata.stamina = (ctx.accounts.clock.unix_timestamp % 10) as u8 + upgradable_metadata.stamina;
        msg!("No current BTC price");
    }

    update_nft_list(
        nft_id,
        ctx.accounts.mint.key(), 
        ctx.accounts.nft_list.clone(),
        &upgradable_metadata
    )?;

    sol_transfer(
        ctx.accounts.owner.to_account_info().clone(),
        ctx.accounts.admin.clone(),
        ctx.accounts.system_program.to_account_info().clone(),
        10000000
    )?;

    emit!(UpgradeEvent {
        owner: ctx.accounts.owner.key(),
        mint: ctx.accounts.mint.key(),
        old_passion,
        old_stamina,
        passion: upgradable_metadata.passion,
        stamina: upgradable_metadata.stamina
    });

    Ok(())
}

pub fn update_nft_list<'info> (
    nft_id: u16,
    nft_mint: Pubkey,
    nft_list: AccountInfo<'info>,
    ex_metadata: &UpgradableMetadata
) -> ProgramResult {

    let start: usize = 2 + nft_id as usize * NFT_ITEM_SIZE;
    let mut nft_list_data = nft_list.data.borrow_mut();

    let nft_pk_inlist: Pubkey = Pubkey::try_from_slice(&nft_list_data[start..start+32])?;
    if nft_mint != nft_pk_inlist {
        return Err(ErrorCode::NftMintMismatch.into());
    }
    nft_list_data[start + 32] = ex_metadata.passion;
    nft_list_data[start + 33] = ex_metadata.stamina;

    Ok(())
}
