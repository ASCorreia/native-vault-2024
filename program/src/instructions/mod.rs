pub mod deposit;
pub mod withdraw;

pub use deposit::*;
pub use withdraw::*;

pub enum VaultInstruction {
    Deposit,
    Withdraw,
}

impl  From<u8> for VaultInstruction {
    fn from(discriminant: u8) -> Self {
        match discriminant {
            0 => Self::Deposit,
            1 => Self::Withdraw,
            _ => panic!("Invalid instruction"),
        }
    }  
}