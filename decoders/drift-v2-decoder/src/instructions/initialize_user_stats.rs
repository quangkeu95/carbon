use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xfef34862fb82a8d5")]
pub struct InitializeUserStats {}

pub struct InitializeUserStatsInstructionAccounts {
    pub user_stats: solana_pubkey::Pubkey,
    pub state: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for InitializeUserStats {
    type ArrangedAccounts = InitializeUserStatsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user_stats, state, authority, payer, rent, system_program, _remaining @ ..] = accounts
        else {
            return None;
        };

        Some(InitializeUserStatsInstructionAccounts {
            user_stats: user_stats.pubkey,
            state: state.pubkey,
            authority: authority.pubkey,
            payer: payer.pubkey,
            rent: rent.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
