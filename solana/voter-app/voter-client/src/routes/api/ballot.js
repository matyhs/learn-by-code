import { getProgram, getConnection, BALLOT_SCHEMA, BallotAccount } from '$lib/scripts/program.js';
import { Keypair, PublicKey, TransactionInstruction, sendAndConfirmTransaction, Transaction } from '@solana/web3.js';
import * as borsh from 'borsh';

export const get = async ({request}) => {
    const data = await request.json();
    const programKey = new PublicKey(data.account);
    const connection = await getConnection();
    const vote = await getBallot(programKey, connection);

    return {
        body: {
            vote
        }
    }
}

export const post = async ({request}) => {
    const data = await request.json();
    const voter = Keypair.fromSecretKey(Buffer.from(request.headers.get('authorization'), "base64"));
    const program = await getProgram();
    const programKey = new PublicKey(data.account);
    const connection = await getConnection();
    const value = Buffer.from(data.value);

    await vote(voter, programKey, program, connection, value);
    const ballotVote = await getBallot(programKey, connection);
    console.log(ballotVote);
    return {
        body: {
            vote: ballotVote
        }

    }
}

const vote = async (sender, receiver, program, connection, data) => {
    const instruction = new TransactionInstruction({ 
        keys: [{ pubkey: receiver, isSigner: false, isWritable: true }],
        programId: program.publicKey,
        data
     });

     await sendAndConfirmTransaction(connection, new Transaction().add(instruction), [sender]);
}

const getBallot = async (programKey, connection) => {
    const account = await connection.getAccountInfo(programKey);
    const data = borsh.deserialize(BALLOT_SCHEMA, BallotAccount, account.data);
    return data.vote;
}