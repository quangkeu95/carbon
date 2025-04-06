use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35")]
pub struct Unverify {
    pub verification_args: VerificationArgs,
}

pub struct UnverifyInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub delegate_record: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_metadata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub sysvar_instructions: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Unverify {
    type ArrangedAccounts = UnverifyInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, delegate_record, metadata, collection_mint, collection_metadata, system_program, sysvar_instructions, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UnverifyInstructionAccounts {
            authority: authority.pubkey,
            delegate_record: delegate_record.pubkey,
            metadata: metadata.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_metadata: collection_metadata.pubkey,
            system_program: system_program.pubkey,
            sysvar_instructions: sysvar_instructions.pubkey,
        })
    }
}
