use {
    super::super::types::*,
    alloc::vec::Vec,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1d")]
pub struct Reallocate {
    pub new_extension_types: Vec<ExtensionType>,
}

pub struct ReallocateInstructionAccounts {
    pub token: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Reallocate {
    type ArrangedAccounts = ReallocateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token, payer, system_program, owner, _remaining @ ..] = accounts else {
            return None;
        };

        Some(ReallocateInstructionAccounts {
            token: token.pubkey,
            payer: payer.pubkey,
            system_program: system_program.pubkey,
            owner: owner.pubkey,
        })
    }
}
