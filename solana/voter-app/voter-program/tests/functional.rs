use {
    solana_program::{
        instruction::{
            AccountMeta, 
            Instruction
        },
        pubkey::Pubkey,
    },
    solana_program_test::*,
    solana_sdk::{
        signature::Signer, 
        transaction::Transaction,
        account::Account
    },
    voter_program::process_instruction
};

#[tokio::test]
async fn test_vote() {
    let program_id = Pubkey::new_unique();
    let voter_pubkey = Pubkey::new_unique();
    let mut program_tests = ProgramTest::new(
        "voting_tests",
        program_id,
        processor!(process_instruction),
    );
    program_tests.add_account(voter_pubkey, Account {
        lamports: 5,
        data: vec![0_u8; std::mem::size_of::<u32>()],
        owner: program_id,
        ..Account::default()
    });

    let (mut banks_client, payer, recent_blockhash) = program_tests.start().await;
    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &["Hello World"],
            vec![AccountMeta::new(voter_pubkey, false)],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();
}