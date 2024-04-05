use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;
pub enum EscrowInstruction {
    /// Starts the trade by creating and populating an escrow account
    /// and transferring ownership of the given temporary token account
    /// to the PDA
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow.
    /// 1. `[writable]` Temporary token account that should be created
    ///     prior to the instruction and owned by the initializer.
    /// 2. `[]` The initializer's token account for the token they will
    ///     receive the trade go through.
    /// 3. `[writable]` The escrow account, it will hold all necessary
    ///     information about the trade.
    /// 4. `[]` The rent sysvar.
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64,
    },
}

impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction]
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        // collect instruction as first item in the array
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        // match instruction and pass `rest` as instruction data
        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount: u64 = input
            .get(..8)
            .and_then(|slice: &[u8]| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
