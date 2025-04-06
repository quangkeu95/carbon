use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xb9adbb5ad80feee9")]
pub struct Disable {
    pub disable_create_pool: bool,
    pub disable_deposit: bool,
    pub disable_withdraw: bool,
    pub disable_buy: bool,
    pub disable_sell: bool,
}

pub struct DisableInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Disable {
    type ArrangedAccounts = DisableInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, global_config, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(DisableInstructionAccounts {
            admin: admin.pubkey,
            global_config: global_config.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
