use anchor_lang::prelude::*;

declare_id!("DBrWpEKgTccMoAQ6umLTSUWsfLR8wYNmvHJSUPGe4XJ8");

pub mod errors;
pub mod utils;
pub mod instructions;
pub mod state;

use instructions::*;

#[program]
pub mod horse_racing {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        operator_list_bump: u8,
        race_bump: u8
    ) -> ProgramResult {
        instructions::initialize::process(ctx, operator_list_bump, race_bump)
    }

    pub fn add_operator(
        ctx: Context<AddOperator>
    ) -> ProgramResult {
        instructions::add_operator::process(ctx)
    }

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        metadata_bump: u8
    ) -> ProgramResult {
        instructions::mint_nft::process(ctx, metadata_bump)
    }

    pub fn upgrade_nft(
        ctx: Context<UpgradeNFT>,
        nft_id: u16
    ) -> ProgramResult {
        instructions::upgrade_nft::process(ctx, nft_id)
    }

    pub fn start_race(
        ctx: Context<StartRace>
    ) -> ProgramResult {
        instructions::start_race::process(ctx)
    }

    pub fn get_award(
        ctx: Context<GetAward>
    ) -> ProgramResult {
        instructions::get_award::process(ctx)
    }
}