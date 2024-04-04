use solana_program::{
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initlizer_pubkey: Pubkey,
    // send tokens from this account to taker
    pub temp_token_account_pubkey: Pubkey,
    // initializer account to receive token
    pub initializer_token_to_receive_account_pubkey: Pubkey,
    // ensure that taker sends enough of the token
    pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}
