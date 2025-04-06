use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x5c892d032d3c75e0")]
pub struct StubOracleClose {}

pub struct StubOracleCloseInstructionAccounts {
    pub owner: solana_pubkey::Pubkey,
    pub oracle: solana_pubkey::Pubkey,
    pub sol_destination: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StubOracleClose {
    type ArrangedAccounts = StubOracleCloseInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [owner, oracle, sol_destination, token_program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(StubOracleCloseInstructionAccounts {
            owner: owner.pubkey,
            oracle: oracle.pubkey,
            sol_destination: sol_destination.pubkey,
            token_program: token_program.pubkey,
        })
    }
}
