use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum LinkedDataKey {
    LinkedLifecycleHook(solana_pubkey::Pubkey),
    LinkedAppData(Authority),
}
