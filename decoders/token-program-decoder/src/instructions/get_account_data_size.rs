use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x15")]
pub struct GetAccountDataSize {}

pub struct GetAccountDataSizeAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for GetAccountDataSize {
    type ArrangedAccounts = GetAccountDataSizeAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.first()?;

        Some(GetAccountDataSizeAccounts { mint: mint.pubkey })
    }
}
