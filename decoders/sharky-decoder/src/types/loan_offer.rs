use {
    super::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct LoanOffer {
    pub lender_wallet: solana_pubkey::Pubkey,
    pub terms_spec: LoanTermsSpec,
    pub offer_time: i64,
}
