extern crate spl_token;

use std::slice::Iter;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    instruction::Instruction,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
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
        let temp_token_account: &AccountInfo = next_account_info(account_info_iter)?;

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
        let mut escrow_info: Escrow = Escrow::unpack_unchecked(&escrow_account.try_borrow_data()?)?;
        if escrow_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // state serialization
        escrow_info.is_initialized = true;
        escrow_info.initializer_pubkey = *initializer.key;
        escrow_info.temp_token_account_pubkey = *temp_token_account.key;
        escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
        escrow_info.expected_amount = amount;
        Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;

        // create a Program Derived Address
        let (pda, _bump_seed): (Pubkey, u8) =
            Pubkey::find_program_address(&[b"escrow"], program_id);

        // collect token program account
        let token_program: &AccountInfo = next_account_info(account_info_iter)?;

        // create account transfer instruction
        let owner_change_ix: Instruction = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");

        // call the token program
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        );

        Ok(())
    }
}
