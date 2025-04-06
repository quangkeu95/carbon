use {
    super::super::types::*,
    carbon_core::{borsh, CarbonDeserialize},
};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1f489fe8dc995a6d")]
pub struct UpdateOrderBook {
    pub order_book_type: Option<OrderBookType>,
    pub apy: Option<APY>,
    pub loan_terms: Option<BookLoanTerms>,
    pub fee_permillicentage: Option<u16>,
    pub fee_authority: Option<solana_pubkey::Pubkey>,
}

pub struct UpdateOrderBookInstructionAccounts {
    pub order_book: solana_pubkey::Pubkey,
    pub payer: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateOrderBook {
    type ArrangedAccounts = UpdateOrderBookInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [order_book, payer, _remaining @ ..] = accounts else {
            return None;
        };

        Some(UpdateOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
        })
    }
}
