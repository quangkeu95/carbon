use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1b")]
pub struct UpdateConfidentialTransferMint {
    pub confidential_transfer_discriminator: u8,
    pub auto_approve_new_accounts: bool,
    pub auditor_elgamal_pubkey: Option<solana_pubkey::Pubkey>,
}

pub struct UpdateConfidentialTransferMintInstructionAccounts {
    pub mint: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateConfidentialTransferMint {
    type ArrangedAccounts = UpdateConfidentialTransferMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [mint, authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateConfidentialTransferMintInstructionAccounts {
            mint: mint.pubkey,
            authority: authority.pubkey,
        })
    }
}
