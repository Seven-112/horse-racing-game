use anchor_lang::prelude::*;

pub const UPGRADABLE_METASIZE: usize = 1 + 1 + 1;
pub const NFT_LIST_SIZE: usize = 2 + (32 + 2) * 1000; // 2 bytes for nft count
pub const OPERATOR_LIST_SIZE: usize = 32 * 10 + 2;
pub const MAX_ADMIN_CNT: usize = 10;
pub const RACE_RESULT_SIZE: usize = 32 + 1;

pub const BTC_DECIMALS: usize = 9;
pub const SOL_DECIMALS: usize = 9;
pub const MIN_PASSION: u8 = 20;
pub const MIN_STAMINA: u8 = 20;

#[account]
pub struct UpgradableMetadata {
    pub bump: u8,
    pub passion: u8,
    pub stamina: u8,
}

#[account]
pub struct OperatorWhiteList {
    pub operator_array: [Pubkey; 10],
    pub operator_cnt: u8,
    pub bump: u8
}

#[account]
pub struct RaceResult {
    pub winner: Pubkey,
    pub bump: u8
}

#[derive(Clone, Copy)]
pub struct Score {
    pub nft_id: u16,
    pub score: u16
}