use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x11")]
pub struct CreateMasterEditionV3 {
    pub create_master_edition_args: CreateMasterEditionArgs,
}

pub struct CreateMasterEditionV3InstructionAccounts {
    pub edition: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub mint_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateMasterEditionV3 {
    type ArrangedAccounts = CreateMasterEditionV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [edition, mint, update_authority, mint_authority, payer, metadata, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateMasterEditionV3InstructionAccounts {
            edition: edition.pubkey,
            mint: mint.pubkey,
            update_authority: update_authority.pubkey,
            mint_authority: mint_authority.pubkey,
            payer: payer.pubkey,
            metadata: metadata.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
