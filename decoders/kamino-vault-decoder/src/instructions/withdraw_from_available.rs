use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1383709baadc2239")]
pub struct WithdrawFromAvailable {
    pub shares_amount: u64,
}

pub struct WithdrawFromAvailableInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub vault_state: solana_pubkey::Pubkey,
    pub token_vault: solana_pubkey::Pubkey,
    pub base_vault_authority: solana_pubkey::Pubkey,
    pub user_token_ata: solana_pubkey::Pubkey,
    pub token_mint: solana_pubkey::Pubkey,
    pub user_shares_ata: solana_pubkey::Pubkey,
    pub shares_mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub shares_token_program: solana_pubkey::Pubkey,
    pub klend_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFromAvailable {
    type ArrangedAccounts = WithdrawFromAvailableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, vault_state, token_vault, base_vault_authority, user_token_ata, token_mint, user_shares_ata, shares_mint, token_program, shares_token_program, klend_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(WithdrawFromAvailableInstructionAccounts {
            user: user.pubkey,
            vault_state: vault_state.pubkey,
            token_vault: token_vault.pubkey,
            base_vault_authority: base_vault_authority.pubkey,
            user_token_ata: user_token_ata.pubkey,
            token_mint: token_mint.pubkey,
            user_shares_ata: user_shares_ata.pubkey,
            shares_mint: shares_mint.pubkey,
            token_program: token_program.pubkey,
            shares_token_program: shares_token_program.pubkey,
            klend_program: klend_program.pubkey,
        })
    }
}
