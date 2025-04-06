use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x00")]
pub struct CreateV1 {
    pub create_v1_args: CreateV1Args,
}

pub struct CreateV1InstructionAccounts {
    pub asset: solana_pubkey::Pubkey,
    pub collection: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub update_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub log_wrapper: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateV1 {
    type ArrangedAccounts = CreateV1InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [asset, collection, authority, payer, owner, update_authority, system_program, log_wrapper, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CreateV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            owner: owner.pubkey,
            update_authority: update_authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}
