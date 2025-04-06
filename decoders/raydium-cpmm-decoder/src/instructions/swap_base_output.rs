use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x37d96256a34ab4ad")]
pub struct SwapBaseOutput {
    pub max_amount_in: u64,
    pub amount_out: u64,
}

pub struct SwapBaseOutputInstructionAccounts {
    pub payer: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub input_token_account: solana_pubkey::Pubkey,
    pub output_token_account: solana_pubkey::Pubkey,
    pub input_vault: solana_pubkey::Pubkey,
    pub output_vault: solana_pubkey::Pubkey,
    pub input_token_program: solana_pubkey::Pubkey,
    pub output_token_program: solana_pubkey::Pubkey,
    pub input_token_mint: solana_pubkey::Pubkey,
    pub output_token_mint: solana_pubkey::Pubkey,
    pub observation_state: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SwapBaseOutput {
    type ArrangedAccounts = SwapBaseOutputInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [payer, authority, amm_config, pool_state, input_token_account, output_token_account, input_vault, output_vault, input_token_program, output_token_program, input_token_mint, output_token_mint, observation_state, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(SwapBaseOutputInstructionAccounts {
            payer: payer.pubkey,
            authority: authority.pubkey,
            amm_config: amm_config.pubkey,
            pool_state: pool_state.pubkey,
            input_token_account: input_token_account.pubkey,
            output_token_account: output_token_account.pubkey,
            input_vault: input_vault.pubkey,
            output_vault: output_vault.pubkey,
            input_token_program: input_token_program.pubkey,
            output_token_program: output_token_program.pubkey,
            input_token_mint: input_token_mint.pubkey,
            output_token_mint: output_token_mint.pubkey,
            observation_state: observation_state.pubkey,
        })
    }
}
