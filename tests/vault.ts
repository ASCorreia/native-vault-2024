import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction, type TransactionInstruction } from '@solana/web3.js';
import { createDepositInstruction, createWithdrawInstruction, PROGRAM_ID } from '../ts';
import BN from 'bn.js';

const commitment: Commitment = 'confirmed';
const connection = new Connection('http://127.0.0.1:8899', commitment);

const user = Keypair.generate();

describe('Solana Native Vault', () => {

    const vault = PublicKey.findProgramAddressSync([Buffer.from('vault'), user.publicKey.toBytes()], PROGRAM_ID)[0];

    it("Airdrop to keypair", async () => {
        let tx = await connection.requestAirdrop(user.publicKey, LAMPORTS_PER_SOL).then((signature) => {
            return connection.confirmTransaction(signature, commitment);
        });
        const balance = await connection.getBalance(user.publicKey);
        console.log("Airdrop Successfull: ", balance);
        console.log("Transaction: ");
    });

    it("Deposit", async () => {
        const amount = LAMPORTS_PER_SOL / 2;
        const instruction = createDepositInstruction(new BN(amount), {
            user: user.publicKey,
            vault: vault,
            systemProgram: SystemProgram.programId,
        });
        const transaction = new Transaction().add(instruction);
        transaction.feePayer = user.publicKey;
        transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

        let tx = await sendAndConfirmTransaction(connection, transaction, [user]);
        const vaultBalance = await connection.getBalance(vault);
        console.log("Deposit Successfull - Vault Balance ", vaultBalance);

        const userBalance = await connection.getBalance(user.publicKey);
        console.log("Deposit Successfull - User Balance ", userBalance);

        console.log("Transaction: ", tx);
    });

    it("Withdraw", async () => {
        const amount = 50000;
        const instruction = createWithdrawInstruction(new BN(amount), {
            user: user.publicKey,
            vault: vault,
            systemProgram: SystemProgram.programId,
        });
        const transaction = new Transaction().add(instruction);
        transaction.feePayer = user.publicKey;
        transaction.recentBlockhash = (await connection.getLatestBlockhash()).blockhash;

        let tx = await sendAndConfirmTransaction(connection, transaction, [user]);
        const vaultBalance = await connection.getBalance(vault);
        console.log("Withdraw Successfull - Vault Balance ", vaultBalance);

        const userBalance = await connection.getBalance(user.publicKey);
        console.log("Withdraw Successfull - User Balance ", userBalance);

        console.log("Transaction: ", tx);
    });

});