use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct CreateMetadataAccount {}

pub struct CreateMetadataAccountInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMetadataAccount {
    type ArrangedAccounts = CreateMetadataAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, mint, mint_authority, payer, update_authority, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateMetadataAccountInstructionAccounts {
            metadata: metadata.pubkey,
            mint: mint.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
