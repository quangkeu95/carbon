use solana_pubkey::Pubkey;

pub struct WeightedSwapDecoder;
pub mod accounts;
pub mod instructions;
pub mod types;

pub const PROGRAM_ID: Pubkey =
    solana_pubkey::Pubkey::from_str_const("swapFpHZwjELNnjvThjajtiVmkz3yPQEHjLtka2fwHW");
