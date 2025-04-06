use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x4f237a54ad0f5dbf")]
pub struct AddImbalanceLiquidity {
    pub minimum_pool_token_amount: u64,
    pub token_a_amount: u64,
    pub token_b_amount: u64,
}

pub struct AddImbalanceLiquidityInstructionAccounts {
    pub pool: solana_pubkey::Pubkey,
    pub lp_mint: solana_pubkey::Pubkey,
    pub user_pool_lp: solana_pubkey::Pubkey,
    pub a_vault_lp: solana_pubkey::Pubkey,
    pub b_vault_lp: solana_pubkey::Pubkey,
    pub a_vault: solana_pubkey::Pubkey,
    pub b_vault: solana_pubkey::Pubkey,
    pub a_vault_lp_mint: solana_pubkey::Pubkey,
    pub b_vault_lp_mint: solana_pubkey::Pubkey,
    pub a_token_vault: solana_pubkey::Pubkey,
    pub b_token_vault: solana_pubkey::Pubkey,
    pub user_a_token: solana_pubkey::Pubkey,
    pub user_b_token: solana_pubkey::Pubkey,
    pub user: solana_pubkey::Pubkey,
    pub vault_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for AddImbalanceLiquidity {
    type ArrangedAccounts = AddImbalanceLiquidityInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [pool, lp_mint, user_pool_lp, a_vault_lp, b_vault_lp, a_vault, b_vault, a_vault_lp_mint, b_vault_lp_mint, a_token_vault, b_token_vault, user_a_token, user_b_token, user, vault_program, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(AddImbalanceLiquidityInstructionAccounts {
            pool: pool.pubkey,
            lp_mint: lp_mint.pubkey,
            user_pool_lp: user_pool_lp.pubkey,
            a_vault_lp: a_vault_lp.pubkey,
            b_vault_lp: b_vault_lp.pubkey,
            a_vault: a_vault.pubkey,
            b_vault: b_vault.pubkey,
            a_vault_lp_mint: a_vault_lp_mint.pubkey,
            b_vault_lp_mint: b_vault_lp_mint.pubkey,
            a_token_vault: a_token_vault.pubkey,
            b_token_vault: b_token_vault.pubkey,
            user_a_token: user_a_token.pubkey,
            user_b_token: user_b_token.pubkey,
            user: user.pubkey,
            vault_program: vault_program.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
