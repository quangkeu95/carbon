use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x64196302d9ef7cad")]
pub struct DeletePositionBundle {}

pub struct DeletePositionBundleInstructionAccounts {
    pub position_bundle: solana_pubkey::Pubkey,
    pub position_bundle_mint: solana_pubkey::Pubkey,
    pub position_bundle_token_account: solana_pubkey::Pubkey,
    pub position_bundle_owner: solana_pubkey::Pubkey,
    pub receiver: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DeletePositionBundle {
    type ArrangedAccounts = DeletePositionBundleInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position_bundle, position_bundle_mint, position_bundle_token_account, position_bundle_owner, receiver, token_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(DeletePositionBundleInstructionAccounts {
            position_bundle: position_bundle.pubkey,
            position_bundle_mint: position_bundle_mint.pubkey,
            position_bundle_token_account: position_bundle_token_account.pubkey,
            position_bundle_owner: position_bundle_owner.pubkey,
            receiver: receiver.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
