import fs from 'mz/fs.js';
import { Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, Transaction, SystemProgram, sendAndConfirmTransaction, TransactionInstruction } from '@solana/web3.js';
import os from 'os';
import path from 'path';
import yaml from 'yaml';
import * as borsh from 'borsh';

class GreetingAccount {
    counter = 0;
    constructor(counter) {
        this.counter = counter;
    }
}

const GREETING_SCHEMA = new Map([[GreetingAccount, {kind: 'struct', fields: [['counter', 'u32']]}]]);
const GREETING_SIZE = borsh.serialize(GREETING_SCHEMA, new GreetingAccount(0)).length;

export const get = async() => {
    const config = await getConfig();
    const connection = getConnection(config.json_rpc_url);

    const fees = await calculateFee(connection);
    const payor = await getPayor(config.keypair_path);
    const balance = await getBalance(connection, payor.publicKey, fees);

    return {
        body: {
            solana: {
                fees,
                payor: payor,
                balance
            }
        }
    }
}

export const post = async ({request}) => {
    const config = await getConfig();
    const connection = getConnection(config.json_rpc_url);
    const payor = await getPayor(config.keypair_path);
    const program = await getProgram();
    const greeted = await getGreetedAccount(connection, payor, program);
    await sayHello(payor, greeted, connection, program);
    await checkHelloCount(greeted, connection);
    return {
        headers: { Location: '/' },
        status: 200
    }
}

const checkHelloCount = async (accountKey, connection) => {
    const account = await connection.getAccountInfo(accountKey);
    const greeting = borsh.deserialize(GREETING_SCHEMA, GreetingAccount, account.data);
    console.log(accountKey.toBase58(), 'has been greeted', greeting.counter, 'time(s)');
}

const sayHello = async (sender, receiver, connection, program) => {
    const instruction = new TransactionInstruction({
        keys: [{ pubkey: receiver, isSigner: false, isWritable: true }],
        programId: program.publicKey,
        data: Buffer.alloc(0)
    });

    await sendAndConfirmTransaction(connection, new Transaction().add(instruction), [sender]);
}

const getPayor = async(keypairPath) => {
    return getKeypairSecret(keypairPath);
}

const calculateFee = async(connection) => {
    let fees = await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);
    fees += LAMPORTS_PER_SOL;
    return fees;
}

const getConnection = (connectionString) => {
    return new Connection(connectionString, 'confirmed');
}

const getConfig = async() => {
    const configPath = path.resolve(os.homedir(), '.config', 'solana', 'cli', 'config.yml');
    const configYml = await fs.readFile(configPath, { encoding: 'utf8' });
    return yaml.parse(configYml);
}

const getBalance = async(connection, publicKey, fees) => {
    let balance = await connection.getBalance(publicKey);

    if (balance < fees) {
        const signature = await connection.requestAirdrop(publicKey, fees - balance);
        await connection.confirmTransaction(signature);
        balance = await connection.getBalance(publicKey);
    }

    return balance;
}

const getProgram = async () => {
    const programPath = path.resolve('..', 'hello-world', 'dist', 'program');
    const programKeypairPath = path.resolve(programPath, 'helloworld-keypair.json');
    return await getKeypairSecret(programKeypairPath);
}

const getGreetedAccount = async (connection, payor, program) => {
    const programInfo = await connection.getAccountInfo(program.publicKey);
    const seed = 'hello';
    const greetedPubkey = await PublicKey.createWithSeed(payor.publicKey, seed, program.publicKey);
    const greetedAccount = await connection.getAccountInfo(greetedPubkey);
    if (greetedAccount === null) {
        const lamport = await connection.getMinimumBalanceForRentExemption(GREETING_SIZE);
        const transaction = new Transaction().add(
            SystemProgram.createAccountWithSeed({
                fromPubkey: payor.publicKey,
                basePubkey: payor.publicKey,
                seed: seed,
                newAccountPubkey: greetedPubkey,
                lamports: lamport,
                space: GREETING_SIZE,
                programId: program.publicKey
            })
        );
        
        await sendAndConfirmTransaction(connection, transaction, [payor]);
    }

    return greetedPubkey;
}

const getKeypairSecret = async (keypairPath) => {
    const secretKeyString = await fs.readFile(keypairPath, { encoding: 'utf8' });
    const secretKey = Uint8Array.from(JSON.parse(secretKeyString));
    return Keypair.fromSecretKey(secretKey);
}