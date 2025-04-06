use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xcf05c8d17a3852b7")]
pub struct SetRewardEmissionsSuperAuthority {}

pub struct SetRewardEmissionsSuperAuthorityInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub reward_emissions_super_authority: solana_pubkey::Pubkey,
    pub new_reward_emissions_super_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardEmissionsSuperAuthority {
    type ArrangedAccounts = SetRewardEmissionsSuperAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, reward_emissions_super_authority, new_reward_emissions_super_authority, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SetRewardEmissionsSuperAuthorityInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            reward_emissions_super_authority: reward_emissions_super_authority.pubkey,
            new_reward_emissions_super_authority: new_reward_emissions_super_authority.pubkey,
        })
    }
}
