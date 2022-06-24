import { serialize } from 'borsh';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import path from 'path';
import fs from 'mz/fs.js';
import os from 'os';
import yaml from 'yaml';

export class BallotAccount {
    vote = '';
    constructor(vote) {
        this.vote = vote;
    }
}

export const BALLOT_SCHEMA = new Map([
                                        [BallotAccount, 
                                            { 
                                                kind: 'struct', 
                                                fields: [
                                                            ['vote', 'string']
                                                        ] 
                                            }
                                        ]
                                    ]);
export const BALLOT_SIZE = serialize(BALLOT_SCHEMA, new BallotAccount('')).length;
export const PROGRAM_SEED = 'vote';

export const getConnection = async () => {
    const config = await getConfig();
    return new Connection(config.json_rpc_url, 'confirmed');
}

export const getProgram = async () => {
    const programPath = path.resolve('..', 'voter-program', 'dist', 'program');
    const programKeyPath = path.resolve(programPath, 'voter_program-keypair.json');
    return await createKeypairFromFile(programKeyPath);
}

export const getBallot = async (voter, program) => {
    return await PublicKey.createWithSeed(voter.publicKey, PROGRAM_SEED, program.publicKey);
}

const getConfig = async () => {
    const configPath = path.resolve(os.homedir(), '.config', 'solana', 'cli', 'config.yml');
    const config = await fs.readFile(configPath, { encoding: 'utf8' });
    return yaml.parse(config);
}

const createKeypairFromFile = async (keyPath) => {
    const secretString = await fs.readFile(keyPath, { encoding: 'utf8' });
    const secret = Uint8Array.from(JSON.parse(secretString));
    return Keypair.fromSecretKey(secret);
}