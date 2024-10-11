use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    program::invoke_signed,
    pubkey::Pubkey
};

use crate::error::VaultError;

pub fn process_withdraw_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [user, vault, _] = accounts
    else {
        return Err(VaultError::NotEnoughAccountKeys.into());
    };

    let vault_pda = Pubkey::find_program_address(&[b"vault", user.key.as_ref()], &crate::ID);
    if vault.key != &vault_pda.0 {
        return Err(VaultError::InvalidAccountData.into());
    }

    let amount: u64 = bytemuck::try_pod_read_unaligned::<u64>(&instruction_data[0..8]).map_err(|_| VaultError::InvalidInstructionData)?;

    let transfer_instruction = solana_program::system_instruction::transfer(vault.key, user.key, amount);
    invoke_signed(
        &transfer_instruction, 
        &[vault.clone(), user.clone()],
        &[&[b"vault", &user.key.to_bytes(), &[vault_pda.1]]],
    )?;

    Ok(())
}