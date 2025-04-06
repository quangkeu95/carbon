use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5d7c10b3f98373f5")]
pub struct InitializePositionBundleWithMetadata {}

pub struct InitializePositionBundleWithMetadataInstructionAccounts {
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bundle_metadata: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_owner: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub metadata_update_auth: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializePositionBundleWithMetadata {
    type ArrangedAccounts = InitializePositionBundleWithMetadataInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_bundle, position_bundle_mint, position_bundle_metadata, position_bundle_token_account, position_bundle_owner, funder, metadata_update_auth, token_program, system_program, rent, associated_token_program, metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializePositionBundleWithMetadataInstructionAccounts {
            position_bundle: position_bundle.pubkey,
            position_bundle_mint: position_bundle_mint.pubkey,
            position_bundle_metadata: position_bundle_metadata.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_owner: position_bundle_owner.pubkey,
            funder: funder.pubkey,
            metadata_update_auth: metadata_update_auth.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
            associated_token_program: associated_token_program.pubkey,
            metadata_program: metadata_program.pubkey,
        })
    }
}
