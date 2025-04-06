use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct SyncNative {}

pub struct SyncNativeAccounts {
    pub account: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SyncNative {
    type ArrangedAccounts = SyncNativeAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let account = accounts.first()?;

        Some(SyncNativeAccounts {
            account: account.pubkey,
        })
    }
}
