use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0e")]
pub struct MintToChecked {
    pub amount: u64,
    pub decimals: u8,
}

pub struct MintToCheckedInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub token: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for MintToChecked {
    type ArrangedAccounts = MintToCheckedInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, token, mint_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(MintToCheckedInstructionAccounts {
            mint: mint.pubkey,
            token: token.pubkey,
            mint_authority: mint_authority.pubkey,
        })
    }
}
