use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x76d7d69db6e5d0e4")]
pub struct SetDefaultFeeRate {
    pub default_fee_rate: u16,
}

pub struct SetDefaultFeeRateInstructionAccounts {
    pub whirlpools_config: solana_pubkey::Pubkey,
    pub fee_tier: solana_pubkey::Pubkey,
    pub fee_authority: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetDefaultFeeRate {
    type ArrangedAccounts = SetDefaultFeeRateInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [whirlpools_config, fee_tier, fee_authority, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetDefaultFeeRateInstructionAccounts {
            whirlpools_config: whirlpools_config.pubkey,
            fee_tier: fee_tier.pubkey,
            fee_authority: fee_authority.pubkey,
        })
    }
}
