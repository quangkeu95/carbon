use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d")]
pub struct BurnNft {}

pub struct BurnNftInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_account: solana_pubkey::Pubkey,
    pub master_edition_account: solana_pubkey::Pubkey,
    pub spl_token_program: solana_pubkey::Pubkey,
    pub collection_metadata: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for BurnNft {
    type ArrangedAccounts = BurnNftInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, owner, mint, token_account, master_edition_account, spl_token_program, collection_metadata, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(BurnNftInstructionAccounts {
            metadata: metadata.pubkey,
            owner: owner.pubkey,
            mint: mint.pubkey,
            token_account: token_account.pubkey,
            master_edition_account: master_edition_account.pubkey,
            spl_token_program: spl_token_program.pubkey,
            collection_metadata: collection_metadata.pubkey,
        })
    }
}
