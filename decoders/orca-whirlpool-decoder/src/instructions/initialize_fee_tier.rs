use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb74a9ca070022a1e")]
pub struct InitializeFeeTier {
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}

pub struct InitializeFeeTierInstructionAccounts {
    pub config: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeFeeTier {
    type ArrangedAccounts = InitializeFeeTierInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [config, fee_tier, funder, fee_authority, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeFeeTierInstructionAccounts {
            config: config.pubkey,
            fee_tier: fee_tier.pubkey,
            funder: funder.pubkey,
            fee_authority: fee_authority.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
