
use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("Invalid Operator to start race")]
    InvalidOperator,

    #[msg("NFT mint is mismatch with NFT pk in list")]
    NftMintMismatch,
}