use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff73dc3a1a9d70b9")]
pub struct TakeLoanV3 {
    pub expected_loan: String,
    pub nft_list_index: Option<u32>,
    pub skip_freezing_collateral: bool,
}

pub struct TakeLoanV3InstructionAccounts {
    pub lender: solana_pubkey::Pubkey,
    pub borrower: solana_pubkey::Pubkey,
    pub borrower_collateral_token_account: solana_pubkey::Pubkey,
    pub collateral_mint: solana_pubkey::Pubkey,
    pub loan: solana_pubkey::Pubkey,
    pub escrow: solana_pubkey::Pubkey,
    pub escrow_collateral_token_account: solana_pubkey::Pubkey,
    pub order_book: solana_pubkey::Pubkey,
    pub metadata: solana_pubkey::Pubkey,
    pub edition: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub mpl_token_metadata_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for TakeLoanV3 {
    type ArrangedAccounts = TakeLoanV3InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [lender, borrower, borrower_collateral_token_account, collateral_mint, loan, escrow, escrow_collateral_token_account, order_book, metadata, edition, system_program, token_program, associated_token_program, rent, mpl_token_metadata_program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(TakeLoanV3InstructionAccounts {
            lender: lender.pubkey,
            borrower: borrower.pubkey,
            borrower_collateral_token_account: borrower_collateral_token_account.pubkey,
            collateral_mint: collateral_mint.pubkey,
            loan: loan.pubkey,
            escrow: escrow.pubkey,
            escrow_collateral_token_account: escrow_collateral_token_account.pubkey,
            order_book: order_book.pubkey,
            metadata: metadata.pubkey,
            edition: edition.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            rent: rent.pubkey,
            mpl_token_metadata_program: mpl_token_metadata_program.pubkey,
        })
    }
}
