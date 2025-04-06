use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct ReservationV1 {
    pub address: solana_pubkey::Pubkey,
    pub spots_remaining: u8,
    pub total_spots: u8,
}
