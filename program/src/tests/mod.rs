use mollusk_svm::{
    program,
    result::ProgramResult,
    Mollusk,
};
use solana_program::{instruction::{AccountMeta, Instruction}, native_token::LAMPORTS_PER_SOL};
use solana_sdk::{account::{AccountSharedData, ReadableAccount}, pubkey::Pubkey};

#[test]
fn deposit() {
    let mollusk: Mollusk = Mollusk::new(&crate::ID, "target/deploy/vault");

    let (system_program, system_program_account) = program::keyed_account_for_system_program();

    let user = Pubkey::new_from_array([0x01; 32]);

    let user_account = AccountSharedData::new(
        10 * LAMPORTS_PER_SOL,
        0,
        &system_program,
    );

    let vault = Pubkey::find_program_address(&[b"vault", user.as_ref()], &crate::ID).0;

    let vault_account = AccountSharedData::new(
        0,
        0,
        &Pubkey::default(),
    );

    let instruction = Instruction::new_with_bytes(
        crate::ID, 
        &[
            &[0x00],
            &(3 * LAMPORTS_PER_SOL).to_le_bytes()[..],
        ]
        .concat(),
        vec![
            AccountMeta::new(user, true),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(system_program, false),
        ],
    );

    let result: mollusk_svm::result::InstructionResult = mollusk.process_instruction(
        &instruction, 
        &vec![
            (user, user_account),
            (vault, vault_account),
            (system_program, system_program_account),
        ],
    );
    // Check value of vault account
    assert_eq!(result.resulting_accounts[0].1.lamports(), 3 * LAMPORTS_PER_SOL);
    assert!(matches!(result.program_result, ProgramResult::Success));
}