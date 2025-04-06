use {
    super::OpenbookV2Decoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod book_side;
pub mod event_heap;
pub mod market;
pub mod open_orders_account;
pub mod open_orders_indexer;
pub mod stub_oracle;

pub enum OpenbookV2Account {
    Market(Box<market::Market>),
    OpenOrdersAccount(Box<open_orders_account::OpenOrdersAccount>),
    OpenOrdersIndexer(open_orders_indexer::OpenOrdersIndexer),
    StubOracle(stub_oracle::StubOracle),
    BookSide(Box<book_side::BookSide>),
    EventHeap(Box<event_heap::EventHeap>),
}

impl AccountDecoder<'_> for OpenbookV2Decoder {
    type AccountType = OpenbookV2Account;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) = market::Market::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::Market(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_account::OpenOrdersAccount::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::OpenOrdersAccount(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            open_orders_indexer::OpenOrdersIndexer::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::OpenOrdersIndexer(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = stub_oracle::StubOracle::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::StubOracle(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = book_side::BookSide::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::BookSide(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = event_heap::EventHeap::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: OpenbookV2Account::EventHeap(Box::new(decoded_account)),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
