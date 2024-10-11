mod state;
mod instructions;
mod error;

use instructions::*;
use solana_program::{account_info::AccountInfo, declare_id, entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};

declare_id!("tjGXp9aPM7WNcMpBqAtDVVkZMjJAACvW1BE9ADgPEeV");


#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if program_id.ne(&ID) {
        return Err(ProgramError::IncorrectProgramId);
    }
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);

    match VaultInstruction::try_from(instruction_discriminant[0]).unwrap() {
        VaultInstruction::Deposit => process_deposit_instruction(accounts, instruction_data_inner)?,
        VaultInstruction::Withdraw => process_withdraw_instruction(accounts, instruction_data)?,  
    }
    
    Ok(())
}
