use solana_program::{
    account_info::AccountInfo, 
    entrypoint::ProgramResult, 
    program::invoke, 
    pubkey::Pubkey
};

use crate::error::VaultError;

pub fn process_deposit_instruction(
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let [user, vault, _] = accounts
    else {
        return Err(VaultError::NotEnoughAccountKeys.into());
    };

    if user.is_signer == false {
        return Err(VaultError::MissingRequiredSignature.into());
    }

    let vault_pda = Pubkey::find_program_address(&[b"vault", user.key.as_ref()], &crate::ID);

    if vault.key != &vault_pda.0 {
        return Err(VaultError::InvalidAccountData.into());
    }

    let amount: u64 = instruction_data.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).unwrap();
    let transfer_ix = solana_program::system_instruction::transfer(user.key, &vault_pda.0, amount);

    invoke(
        &transfer_ix, 
        &[user.clone(), vault.clone()]
    )  
}