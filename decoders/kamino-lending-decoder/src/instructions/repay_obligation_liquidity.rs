

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x91b20de14cf09348")]
pub struct RepayObligationLiquidity{
    pub liquidity_amount: u64,
}

pub struct RepayObligationLiquidityInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub repay_reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_liquidity_mint: solana_sdk::pubkey::Pubkey,
    pub reserve_destination_liquidity: solana_sdk::pubkey::Pubkey,
    pub user_source_liquidity: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RepayObligationLiquidity {
    type ArrangedAccounts = RepayObligationLiquidityInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_sdk::instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let repay_reserve = accounts.get(3)?;
        let reserve_liquidity_mint = accounts.get(4)?;
        let reserve_destination_liquidity = accounts.get(5)?;
        let user_source_liquidity = accounts.get(6)?;
        let token_program = accounts.get(7)?;
        let instruction_sysvar_account = accounts.get(8)?;

        Some(RepayObligationLiquidityInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            repay_reserve: repay_reserve.pubkey,
            reserve_liquidity_mint: reserve_liquidity_mint.pubkey,
            reserve_destination_liquidity: reserve_destination_liquidity.pubkey,
            user_source_liquidity: user_source_liquidity.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}