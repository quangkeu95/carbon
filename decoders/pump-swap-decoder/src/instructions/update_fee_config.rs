use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x68b867f258976b14")]
pub struct UpdateFeeConfig {
    pub lp_fee_basis_points: u64,
    pub protocol_fee_basis_points: u64,
    pub protocol_fee_recipients: [solana_pubkey::Pubkey; 8],
}

pub struct UpdateFeeConfigInstructionAccounts {
    pub admin: solana_pubkey::Pubkey,
    pub global_config: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeeConfig {
    type ArrangedAccounts = UpdateFeeConfigInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [admin, global_config, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateFeeConfigInstructionAccounts {
            admin: admin.pubkey,
            global_config: global_config.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
