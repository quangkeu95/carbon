use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x13")]
pub struct Utilize {
    pub utilize_args: UtilizeArgs,
}

pub struct UtilizeInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub use_authority: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub ata_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub use_authority_record: solana_pubkey::Pubkey,
    pub burner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Utilize {
    type ArrangedAccounts = UtilizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, token_account, mint, use_authority, owner, token_program, ata_program, system_program, rent, use_authority_record, burner, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(UtilizeInstructionAccounts {
            metadata: metadata.pubkey,
            token_account: token_account.pubkey,
            mint: mint.pubkey,
            use_authority: use_authority.pubkey,
            owner: owner.pubkey,
            token_program: token_program.pubkey,
            ata_program: ata_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            use_authority_record: use_authority_record.pubkey,
            burner: burner.pubkey,
        })
    }
}
