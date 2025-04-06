use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x19")]
pub struct SetAndVerifyCollection {}

pub struct SetAndVerifyCollectionInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub collection_authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub collection_mint: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub collection_master_edition_account: solana_pubkey::Pubkey,
    pub collection_authority_record: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetAndVerifyCollection {
    type ArrangedAccounts = SetAndVerifyCollectionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, collection_authority, payer, update_authority, collection_mint, collection, collection_master_edition_account, collection_authority_record, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetAndVerifyCollectionInstructionAccounts {
            metadata: metadata.pubkey,
            collection_authority: collection_authority.pubkey,
            payer: payer.pubkey,
            update_authority: update_authority.pubkey,
            collection_mint: collection_mint.pubkey,
            collection: collection.pubkey,
            collection_master_edition_account: collection_master_edition_account.pubkey,
            collection_authority_record: collection_authority_record.pubkey,
        })
    }
}
