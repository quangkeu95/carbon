use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct InitializeMint {
    pub decimals: u8,
    pub mint_authority: solana_pubkey::Pubkey,
    // TODO: It's COPTION on github so don't know
    pub freeze_authority: Option<solana_pubkey::Pubkey>,
}

pub struct InitializeMintAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeMint {
    type ArrangedAccounts = InitializeMintAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(InitializeMintAccounts {
            mint: mint.pubkey,
            rent: rent.pubkey,
        })
    }
}
