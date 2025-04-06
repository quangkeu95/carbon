use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct Oracle {
    pub base_address: solana_pubkey::Pubkey,
    pub base_address_config: Option<ExtraAccount>,
    pub results_offset: ValidationResultsOffset,
}
