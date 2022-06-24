import { getBallot, getConnection, getProgram } from '$lib/scripts/program.js';
import { Keypair, LAMPORTS_PER_SOL, sendAndConfirmTransaction, SystemProgram, Transaction } from '@solana/web3.js';

export const get = async ({request}) => {
    const voter = Keypair.fromSecretKey(Buffer.from(request.headers.get('authorization'), "base64"));
    const program = await getProgram();
    const ballot = await getBallot(voter, program);
    const account = await verifyAccount(ballot);

    if (account === null) {
        return {
            status: 404,
            body: {
                error: 'Account not found'
            }
        }
    }

    return {
        body: {
            account: ballot.toBase58()
        }
    };
}

export const post = async ({request}) => {
    const voter = Keypair.fromSecretKey(Buffer.from(request.headers.get('authorization'), "base64"));
    const program = await getProgram();
    const ballot = await getBallot(voter, program);
    await createAccount(voter, ballot);

    return {
        body: {
            ballot: ballot.toBase58()
        }
    };
}

const createAccount = async (voter, ballotKey) => {
    const connection = await getConnection();
    const fee = await connection.getMinimumBalanceForRentExemption(BALLOT_SIZE);
    const balance = await connection.getBalance(voter.publicKey);
    if (balance < fee) {
        const signature = await connection.requestAirdrop(voter.publicKey, (fee + LAMPORTS_PER_SOL) - balance);
        await connection.confirmTransaction(signature);
    }

    const transaction = new Transaction().add(
        SystemProgram.createAccountWithSeed({
            fromPubkey: voter.publicKey,
            basePubkey: voter.publicKey,
            seed: PROGRAM_SEED,
            newAccountPubkey: ballotKey,
            lamports: fee,
            space: BALLOT_SIZE,
            programId: program.publicKey
        })
    );
    await sendAndConfirmTransaction(connection, transaction, [voter]);
}

const verifyAccount = async (ballot) => {
    const connection = await getConnection();
    return await connection.getAccountInfo(ballot);
}