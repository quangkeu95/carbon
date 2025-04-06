use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x22")]
pub struct SetCollectionSize {
    pub set_collection_size_args: SetCollectionSizeArgs,
}

pub struct SetCollectionSizeInstructionAccounts {
    pub collection_metadata: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection_authority_record: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetCollectionSize {
    type ArrangedAccounts = SetCollectionSizeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [collection_metadata, collection_authority, collection_mint, collection_authority_record, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetCollectionSizeInstructionAccounts {
            collection_metadata: collection_metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}
