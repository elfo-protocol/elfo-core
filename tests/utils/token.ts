import {Provider} from '@project-serum/anchor';
import {
    createAssociatedTokenAccountInstruction,
    createInitializeMintInstruction,
    createMintToInstruction,
    getAssociatedTokenAddress,
    TOKEN_PROGRAM_ID,
} from "@solana/spl-token";


import {Keypair, PublicKey, SystemProgram, Transaction, TransactionInstruction} from '@solana/web3.js';

export async function createMint(
    provider: Provider,
    authority?: PublicKey,
    decimals?: number,
): Promise<PublicKey> {
    if (authority === undefined) {
        authority = provider.wallet.publicKey;
    }
    const mint = Keypair.generate();
    const instructions = await createMintInstructions(
        provider,
        authority,
        mint.publicKey,
        decimals,
    );

    const tx = new Transaction();
    tx.add(...instructions);

    await provider.send(tx, [mint]);

    return mint.publicKey;
}

async function createMintInstructions(
    provider: Provider,
    authority: PublicKey,
    mint: PublicKey,
    decimals?: number,
): Promise<TransactionInstruction[]> {
    return [
        SystemProgram.createAccount({
            fromPubkey: provider.wallet.publicKey,
            newAccountPubkey: mint,
            space: 82,
            lamports: await provider.connection.getMinimumBalanceForRentExemption(82),
            programId: TOKEN_PROGRAM_ID,
        }),
        createInitializeMintInstruction(
            mint,
            decimals,
            authority,
            null,
            TOKEN_PROGRAM_ID
        ),
    ];
}


export async function createAssocciatedTokenAccount(
    provider: Provider,
    mint: PublicKey,
    owner: PublicKey,
): Promise<PublicKey> {
    const associatedAccount = await getAssociatedTokenAddress(mint, owner);

    const tx = new Transaction(
        {
            feePayer: provider.wallet.publicKey,
            recentBlockhash: (await provider.connection.getLatestBlockhash()).blockhash,
        }
    );
    tx.add(
        createAssociatedTokenAccountInstruction(
            provider.wallet.publicKey,
            associatedAccount,
            owner,
            mint
        )
    );
    await provider.wallet.signTransaction(tx);

    await provider.send(tx);
    return associatedAccount;
}

export async function mintUSDC(
    provider: Provider,
    mint: PublicKey,
    destination: PublicKey,
    mint_authority: PublicKey,
    amount: number,
) {
    const instructions = await createMintToInstruction(
        mint,
        destination,
        mint_authority,
        amount,
    );

    const tx = new Transaction({
        feePayer: provider.wallet.publicKey,
        recentBlockhash: (await provider.connection.getLatestBlockhash()).blockhash,
    }).add(instructions);

    await provider.wallet.signTransaction(tx);
    await provider.send(tx);
}