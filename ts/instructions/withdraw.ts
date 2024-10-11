import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import BN from "bn.js";
import { PROGRAM_ID } from "../";

export type createWithdrawInstructionAccounts = {
    user: PublicKey,
    vault: PublicKey,
    systemProgram: PublicKey,
};

export function createWithdrawInstruction(amount: BN, accounts: createWithdrawInstructionAccounts): TransactionInstruction {
    return new TransactionInstruction({
        programId: PROGRAM_ID,
        keys: [
            { 
                pubkey: accounts.user, 
                isSigner: true, 
                isWritable: true 
            },
            { pubkey: 
                accounts.vault, 
                isSigner: false, 
                isWritable: true 
            },
            { 
                pubkey: accounts.systemProgram, 
                isSigner: false, 
                isWritable: false 
            },
        ],
        data: Buffer.from([0x1, ...amount.toArray("le", 8)]),
    });
}