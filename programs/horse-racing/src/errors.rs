
use anchor_lang::error;

#[error]
pub enum ErrorCode {
    #[msg("Invalid Operator to start race")]
    InvalidOperator,

    #[msg("Operator Count is Maximum!")]
    OperatorOverflow,

    #[msg("No Operator to remove")]
    OperatorNotEnough,

    #[msg("Operator is already existing")]
    OperatorDuplicated,

    #[msg("Operator not found in operator_list")]
    OperatorNotFound,
    
    #[msg("Only Operator can be an Admin!")]
    TransferRoleToNormalUser,

    #[msg("Admin's key is wrong.")]
    UnknownAdmin,

    #[msg("NFT mint is mismatch with NFT pk in list")]
    NftMintMismatch,

}