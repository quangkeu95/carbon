use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5f87c0c4f281e644")]
pub struct InitializeReward {
    pub reward_index: u8,
}

pub struct InitializeRewardInstructionAccounts {
    pub reward_authority: solana_pubkey::Pubkey,
    pub funder: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub reward_mint: solana_pubkey::Pubkey,
    pub reward_vault: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeReward {
    type ArrangedAccounts = InitializeRewardInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [reward_authority, funder, whirlpool, reward_mint, reward_vault, token_program, system_program, rent, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(InitializeRewardInstructionAccounts {
            reward_authority: reward_authority.pubkey,
            funder: funder.pubkey,
            whirlpool: whirlpool.pubkey,
            reward_mint: reward_mint.pubkey,
            reward_vault: reward_vault.pubkey,
            token_program: token_program.pubkey,
            system_program: system_program.pubkey,
            rent: rent.pubkey,
        })
    }
}
