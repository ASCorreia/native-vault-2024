use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_error::ProgramError, pubkey::Pubkey};

pub fn process_deposit_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [user, vault] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    if user.is_signer == false {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let vault_pda = Pubkey::find_program_address(&[b"vault", user.key.as_ref()], &crate::ID);

    if vault.key != &vault_pda.0 {
        return Err(ProgramError::InvalidAccountData);
    }

    let amount: u64 = instruction_data.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).unwrap();
    let transfer_ix = solana_program::system_instruction::transfer(user.key, &vault_pda.0, amount);

    invoke(
        &transfer_ix, 
        &[user.clone(), vault.clone()]
    )  
}