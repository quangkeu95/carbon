use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x17")]
pub struct AmountToUiAmount {
    pub amount: u64,
}

pub struct AmountToUiAmountAccounts {
    pub mint: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AmountToUiAmount {
    type ArrangedAccounts = AmountToUiAmountAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mint = accounts.first()?;

        Some(AmountToUiAmountAccounts { mint: mint.pubkey })
    }
}
