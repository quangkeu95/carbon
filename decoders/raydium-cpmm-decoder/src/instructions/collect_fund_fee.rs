use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xa78a4e95dfc2067e")]
pub struct CollectFundFee {
    pub amount0_requested: u64,
    pub amount1_requested: u64,
}

pub struct CollectFundFeeInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub pool_state: solana_pubkey::Pubkey,
    pub amm_config: solana_pubkey::Pubkey,
    pub token0_vault: solana_pubkey::Pubkey,
    pub token1_vault: solana_pubkey::Pubkey,
    pub vault0_mint: solana_pubkey::Pubkey,
    pub vault1_mint: solana_pubkey::Pubkey,
    pub recipient_token0_account: solana_pubkey::Pubkey,
    pub recipient_token1_account: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub token_program2022: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CollectFundFee {
    type ArrangedAccounts = CollectFundFeeInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, authority, pool_state, amm_config, token0_vault, token1_vault, vault0_mint, vault1_mint, recipient_token0_account, recipient_token1_account, token_program, token_program2022, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CollectFundFeeInstructionAccounts {
            owner: owner.pubkey,
            authority: authority.pubkey,
            pool_state: pool_state.pubkey,
            amm_config: amm_config.pubkey,
            token0_vault: token0_vault.pubkey,
            token1_vault: token1_vault.pubkey,
            vault0_mint: vault0_mint.pubkey,
            vault1_mint: vault1_mint.pubkey,
            recipient_token0_account: recipient_token0_account.pubkey,
            recipient_token1_account: recipient_token1_account.pubkey,
            token_program: token_program.pubkey,
            token_program2022: token_program2022.pubkey,
        })
    }
}
