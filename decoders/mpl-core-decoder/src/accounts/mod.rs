use {
    super::MplCoreProgramDecoder,
    crate::PROGRAM_ID,
    carbon_core::{account::AccountDecoder, deserialize::CarbonDeserialize},
};
pub mod asset_v1;
pub mod collection_v1;
pub mod hashed_asset_v1;
pub mod plugin_header_v1;
pub mod plugin_registry_v1;

pub enum MplCoreProgramAccount {
    PluginHeaderV1(plugin_header_v1::PluginHeaderV1),
    PluginRegistryV1(plugin_registry_v1::PluginRegistryV1),
    AssetV1(asset_v1::AssetV1),
    CollectionV1(collection_v1::CollectionV1),
    HashedAssetV1(hashed_asset_v1::HashedAssetV1),
}

impl AccountDecoder<'_> for MplCoreProgramDecoder {
    type AccountType = MplCoreProgramAccount;
    fn decode_account(
        &self,
        account: &solana_account::Account,
    ) -> Option<carbon_core::account::DecodedAccount<Self::AccountType>> {
        if !account.owner.eq(&PROGRAM_ID) {
            return None;
        }

        if let Some(decoded_account) =
            plugin_header_v1::PluginHeaderV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MplCoreProgramAccount::PluginHeaderV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            plugin_registry_v1::PluginRegistryV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MplCoreProgramAccount::PluginRegistryV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) = asset_v1::AssetV1::deserialize(account.data.as_slice()) {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MplCoreProgramAccount::AssetV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            collection_v1::CollectionV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MplCoreProgramAccount::CollectionV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        if let Some(decoded_account) =
            hashed_asset_v1::HashedAssetV1::deserialize(account.data.as_slice())
        {
            return Some(carbon_core::account::DecodedAccount {
                lamports: account.lamports,
                data: MplCoreProgramAccount::HashedAssetV1(decoded_account),
                owner: account.owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
            });
        }

        None
    }
}
