use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(CarbonDeserialize, Debug)]
pub struct CollectionAuthorityRecord {
    pub key: Key,
    pub bump: u8,
    pub update_authority: Option<solana_pubkey::Pubkey>,
}
