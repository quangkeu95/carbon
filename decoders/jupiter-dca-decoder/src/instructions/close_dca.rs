use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x16072162a8b722f3")]
pub struct CloseDca {}

#[derive(Debug, PartialEq)]
pub struct CloseDcaInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub dca: solana_pubkey::Pubkey,
    pub input_mint: solana_pubkey::Pubkey,
    pub output_mint: solana_pubkey::Pubkey,
    pub in_ata: solana_pubkey::Pubkey,
    pub out_ata: solana_pubkey::Pubkey,
    pub user_in_ata: solana_pubkey::Pubkey,
    pub user_out_ata: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseDca {
    type ArrangedAccounts = CloseDcaInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [user, dca, input_mint, output_mint, in_ata, out_ata, user_in_ata, user_out_ata, system_program, token_program, associated_token_program, event_authority, program, _remaining @ ..] =
            accounts
        else {
            return None;
        };

        Some(CloseDcaInstructionAccounts {
            user: user.pubkey,
            dca: dca.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            user_in_ata: user_in_ata.pubkey,
            user_out_ata: user_out_ata.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
