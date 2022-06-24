use {
    solana_program::{
        entrypoint,
        pubkey::Pubkey,
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program_error::ProgramError,
    },
    borsh::{
        BorshDeserialize,
        BorshSerialize
    }
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct BallotAccount {
    pub vote: String
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let account = next_account_info(&mut accounts.iter())?;

    if account.owner != program_id {
        msg!("Voter account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    let vote = std::str::from_utf8(instruction_data).unwrap();
    let mut ballot_account = BallotAccount::try_from_slice(&account.data.borrow())?;
    ballot_account.vote = String::from(vote);
    
    msg!("voted for {}", ballot_account.vote);

    Ok(())
}