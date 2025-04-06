use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x35f38941088c9e06")]
pub struct SetFeeRate {
    pub fee_rate: u16,
}

pub struct SetFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub whirlpool: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetFeeRate {
    type ArrangedAccounts = SetFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, whirlpool, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            whirlpool: whirlpool.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
