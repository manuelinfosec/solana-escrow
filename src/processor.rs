extern crate spl_token;

use std::slice::Iter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::EscrowError, instruction::EscrowInstruction, state::Escrow};

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction: EscrowInstruction = EscrowInstruction::unpack(instruction_data)?;

        match instruction {
            EscrowInstruction::InitEscrow { amount } => {
                msg!("Instruction: InitEscrow");
                Self::process_init_escrow(accounts, amount, program_id)
            }
        }
    }

    fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter: &mut Iter<'_, AccountInfo> = &mut accounts.iter();
        let initializer: &AccountInfo = next_account_info(account_info_iter)?;

        // initializer of the escrow has to sign the transaction
        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        // X account where init'er sends tokens to exchange
        let temp_account_token: &AccountInfo = next_account_info(account_info_iter)?;

        // Y account where taker sends tokens
        let token_to_receive_account: &AccountInfo = next_account_info(account_info_iter)?;

        // check if account is owned by the token program
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let escrow_account: &AccountInfo = next_account_info(account_info_iter)?;

        // collect rent to be paid by the account
        let rent: Rent = Rent::from_account_info(next_account_info(account_info_iter)?)?;

        // if account is not rent exempted
        if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
            return Err(EscrowError::NotRentExempt.into());
        }

        // Deserialize data from in the escrow account [u8]
        let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        Ok(())
    }
}
