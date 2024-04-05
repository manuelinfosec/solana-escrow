use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    // Invalid instruction
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Rent exempted")]
    NotRentExempt,
}

// function should return user-defined error for `.into()`
impl From<EscrowError> for ProgramError {
    fn from(e: EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
