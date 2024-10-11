import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import { PROGRAM_ID } from "../";

export type createDepositInstructionAccounts = {
    user: PublicKey,
    vault: PublicKey,
    systemProgram: PublicKey,
};

export function createDepositInstruction(amount: BN, accounts: createDepositInstructionAccounts): TransactionInstruction {
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { 
                pubkey: accounts.user, 
                isSigner: true, 
                isWritable: true 
            },
            {   pubkey: accounts.vault, 
                isSigner: false, 
                isWritable: true 
            },
            { 
                pubkey: accounts.systemProgram, 
                isSigner: false, 
                isWritable: false 
            },
        ],
        data: Buffer.from([0x0, ...amount.toArray("le", 8)]),
    });
}