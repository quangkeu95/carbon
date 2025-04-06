use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1c")]
pub struct RemoveCreatorVerification {}

pub struct RemoveCreatorVerificationInstructionAccounts {
    pub metadata: solana_pubkey::Pubkey,
    pub creator: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for RemoveCreatorVerification {
    type ArrangedAccounts = RemoveCreatorVerificationInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [metadata, creator, _remaining @ ..] = accounts else {
            return None;
        };

        Some(RemoveCreatorVerificationInstructionAccounts {
            metadata: metadata.pubkey,
            creator: creator.pubkey,
        })
    }
}
