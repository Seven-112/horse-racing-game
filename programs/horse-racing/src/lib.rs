use anchor_lang::prelude::*;
use anchor_spl::token::{self, TokenAccount, Transfer};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    sysvar::{clock::Clock},
    msg
};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

pub const UPGRADABLE_METASIZE: usize = 1 + 1 + 1;
pub const BTC_DECIMALS: usize = 9;
pub const SOL_DECIMALS: usize = 9;
pub const MIN_PASSION: u8 = 20;
pub const MIN_STAMINA: u8 = 20;

#[program]
pub mod horse_racing {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
    pub fn mint_nft(
        ctx: Context<MintNFT>,
        _bump: u8
    ) -> ProgramResult {
        let clock = Clock::from_account_info(&ctx.accounts.clock)?;

        let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;
        upgradable_metadata.bump = _bump;
        upgradable_metadata.passion = 0;
        upgradable_metadata.stamina = 0;

        let sol_price = chainlink::get_price(&chainlink::id(), ctx.accounts.sol_feed_account)?;
        let btc_price = chainlink::get_price(&chainlink::id(), ctx.accounts.btc_feed_account)?;
        if let Some(sol_price) = sol_price {
            let rand_from_sol = sol_price as u64 + clock.unix_timestamp as u64;
            upgradable_metadata.passion = (rand_from_sol % 10) as u8 + MIN_PASSION;
            msg!("Sol Price is {}", sol_price);
        } else {
            upgradable_metadata.passion = (clock.unix_timestamp % 10) as u8 + MIN_PASSION;
            msg!("No current Sol price");
        }

        if let Some(btc_price) = btc_price {
            let rand_from_sol = btc_price as u64 + clock.unix_timestamp as u64;
            upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + MIN_STAMINA;
            msg!("BTC Price is {}", btc_price);
        } else {
            upgradable_metadata.stamina = (clock.unix_timestamp % 10) as u8 + MIN_STAMINA;
            msg!("No current BTC price");
        }

        Ok(())
    }

    pub fn upgrade_nft(
        ctx: Context<UpgradeNFT>
    ) -> ProgramResult {
        let clock = Clock::from_account_info(&ctx.accounts.clock)?;
        let upgradable_metadata = &mut ctx.accounts.upgradable_metadata;

        let sol_price = chainlink::get_price(&chainlink::id(), ctx.accounts.sol_feed_account)?;
        let btc_price = chainlink::get_price(&chainlink::id(), ctx.accounts.btc_feed_account)?;
        if let Some(sol_price) = sol_price {
            let rand_from_sol = sol_price as u64 + clock.unix_timestamp as u64;
            upgradable_metadata.passion = (rand_from_sol % 10) as u8 + upgradable_metadata.passion;
            msg!("Sol Price is {}", sol_price);
        } else {
            upgradable_metadata.passion = (clock.unix_timestamp % 10) as u8 + upgradable_metadata.passion;
            msg!("No current Sol price");
        }

        if let Some(btc_price) = btc_price {
            let rand_from_sol = btc_price as u64 + clock.unix_timestamp as u64;
            upgradable_metadata.stamina = (rand_from_sol % 10) as u8 + upgradable_metadata.stamina;
            msg!("BTC Price is {}", btc_price);
        } else {
            upgradable_metadata.stamina = (clock.unix_timestamp % 10) as u8 + upgradable_metadata.stamina;
            msg!("No current BTC price");
        }

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
    #[account(mut)]
    pub admin: Signer<'info>,
}

#[derive(Accounts)]
#[instruction(_bump : u8)]
pub struct MintNFT<'info> {
    #[account(mut, signer)]
    pub admin: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [(*owner.key).as_ref(), (*mint.key).as_ref()],
        bump = _bump,
        payer = owner,
        space = 8 + UPGRADABLE_METASIZE
    )]
    pub upgradable_metadata: Account<'info, UpgradableMetadata>,

    #[account(owner = spl_token::id())]
    pub mint: AccountInfo<'info>,

    #[account(
        constraint = token_account.owner == *mint.key,
        constraint = *token_account.to_account_info().owner == token::Token::id()
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub sol_feed_account: AccountInfo<'info>,

    pub btc_feed_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

    pub clock: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct UpgradeNFT<'info> {
    #[account(mut, signer)]
    pub owner: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [(*owner.key).as_ref(), (*mint.key).as_ref()],
        bump = upgradable_metadata.bump
    )]
    pub upgradable_metadata: ProgramAccount<'info, UpgradableMetadata>,

    #[account(owner = spl_token::id())]
    pub mint: AccountInfo<'info>,

    #[account(
        constraint = token_account.owner == *mint.key,
        constraint = *token_account.to_account_info().owner == token::Token::id()
    )]
    pub token_account: Account<'info, TokenAccount>,

    pub sol_feed_account: AccountInfo<'info>,

    pub btc_feed_account: AccountInfo<'info>,

    #[account(owner = spl_token::id())]
    pub token_program: AccountInfo<'info>,

    pub clock: AccountInfo<'info>
}

#[account]
pub struct UpgradableMetadata {
    pub bump: u8,
    pub passion: u8,
    pub stamina: u8,
}
