use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0dc556a86db01bf4")]
pub struct SetRewardEmissions {
    pub reward_index: u8,
    pub emissions_per_second_x64: u128,
}

pub struct SetRewardEmissionsInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_authority: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardEmissions {
    type ArrangedAccounts = SetRewardEmissionsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, reward_authority, reward_vault, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetRewardEmissionsInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            reward_authority: reward_authority.pubkey,
            reward_vault: reward_vault.pubkey,
        })
    }
}
