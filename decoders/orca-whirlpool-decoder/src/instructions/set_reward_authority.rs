use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x2227b7fc531c557f")]
pub struct SetRewardAuthority {
    pub reward_index: u8,
}

pub struct SetRewardAuthorityInstructionAccounts {
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_authority: solana_pubkey::Pubkey,
    pub new_reward_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetRewardAuthority {
    type ArrangedAccounts = SetRewardAuthorityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpool, reward_authority, new_reward_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetRewardAuthorityInstructionAccounts {
            whirlpool: whirlpool.pubkey,
            reward_authority: reward_authority.pubkey,
            new_reward_authority: new_reward_authority.pubkey,
        })
    }
}
