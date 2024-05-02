use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
    pub is_initialized: bool,
    pub initializer_pubkey: Pubkey,
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

// Deserialization trait
impl Pack for Escrow {
    // we know how much data would be unpacked (in bytes)
    const LEN: usize = 105;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        // create a slice of exactly 105 bytes starting from the 0th index (that should be all the data)
        let src: &[u8; 105] = array_ref![src, 0, Escrow::LEN];

        // collect various slices of `src` based on the size specified
        let (
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initialzer_token_to_receive_account_pubkey,
            expected_amount,
        ): (&[u8; 1], &[u8; 32], &[u8; 32], &[u8; 32], &[u8; 8]) =
            array_refs![src, 1, 32, 32, 32, 8];

        // cast u8 to bool
        let is_initialized: bool = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        // construct escrow state
        Ok(Escrow {
            is_initialized,
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
            initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(
                *initialzer_token_to_receive_account_pubkey,
            ),
            expected_amount: u64::from_le_bytes(*expected_amount),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst: &mut [u8; 105] = array_mut_ref![dst, 0, Escrow::LEN]
        let (
            is_initialized_dst,
            initializer_pubkey_dst,
            temp_token_account_pubkey_dst,
            initializer_token_to_receive_account_pubkey_dst,
            expected_amount_dst
        ) = mut_array_refs![dst, 1, 32, 32, 32, 8];

        let Escrow {
            is_initialized,
            initializer_pubkey,
            temp_token_account_pubkey,
            initializer_token_to_receive_account_pubkey,
            expected_amount
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;  
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
        initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
        *expected_amount_dst = expected_amount.to_le_bytes()
    }
}
