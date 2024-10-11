use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum VaultError {
    #[error("Not enough account keys")]
    NotEnoughAccountKeys,
    #[error("Invalid account data")]
    InvalidAccountData,
    #[error("Invalid instruction data")]
    InvalidInstructionData,
    #[error("Incorrect program ID")]
    IncorrectProgramId,
    #[error("Missing required signature")]
    MissingRequiredSignature,
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}