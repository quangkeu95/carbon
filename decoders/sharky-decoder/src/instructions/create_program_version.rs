use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x67d800ee5c6bdb79")]
pub struct CreateProgramVersion {
    pub version: u8,
}

pub struct CreateProgramVersionInstructionAccounts {
    pub authority: solana_pubkey::Pubkey,
    pub program_version: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateProgramVersion {
    type ArrangedAccounts = CreateProgramVersionInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [authority, program_version, system_program, rent, _remaining @ ..] = accounts else {
            return None;
        };

        Some(CreateProgramVersionInstructionAccounts {
            authority: authority.pubkey,
            program_version: program_version.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
