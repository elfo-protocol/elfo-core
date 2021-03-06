import * as anchor from '@project-serum/anchor';
import { Program, Provider, BN, Wallet } from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from '@solana/spl-token';

import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY
} from '@solana/web3.js';
import { assert } from 'chai';
import { ElfoProtocol } from '../target/types/elfo_protocol';
import { createMint, createAssocciatedTokenAccount, mintUSDC } from './utils/token';

const utf8 = anchor.utils.bytes.utf8;

describe('[elfo-protocol] - Negative Test Cases', () => {
    const provider = Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.ElfoProtocol as Program<ElfoProtocol>;
    const environmentWallet = program.provider.wallet;

    let mint: PublicKey, mint_decimals = 6;
    let subscriptionPlanAuthorWallet: Wallet, subscriptionPaymentUSDCAssociatedAccount: PublicKey;
    let subscriberWallet: Wallet, subscriberUSDCAssociatedAccount: PublicKey;

    let protocolState: PublicKey, protocolSigner: PublicKey;

    let protocolSignerBump: number,
    protocolStateBump : number;

    before('Boilerplates', async () => {
        // Creating a wallet for subscription author
        subscriptionPlanAuthorWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(subscriptionPlanAuthorWallet.publicKey, 1000 * LAMPORTS_PER_SOL);

        // Creating wallets for subscribers
        subscriberWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(subscriberWallet.publicKey, 1000 * LAMPORTS_PER_SOL)

        // Creating a dummy USDC mint
        mint = await createMint(provider, environmentWallet.publicKey, mint_decimals);

        // Creating subscription author payment account with 5000 USDC
        subscriptionPaymentUSDCAssociatedAccount = await createAssocciatedTokenAccount(provider, mint, subscriptionPlanAuthorWallet.publicKey);
        await mintUSDC(provider, mint, subscriptionPaymentUSDCAssociatedAccount, environmentWallet.publicKey, 5000 * Math.pow(10, mint_decimals))

        // Createing subscriber token payment account with 5000 USDC
        subscriberUSDCAssociatedAccount = await createAssocciatedTokenAccount(provider, mint, subscriberWallet.publicKey);
        await mintUSDC(provider, mint, subscriberUSDCAssociatedAccount, environmentWallet.publicKey, 5000 * Math.pow(10, mint_decimals));

        // Initialize protocol
        [protocolSigner, protocolSignerBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("protocol_signer")
            ],
            program.programId
        );

        [protocolState, protocolStateBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("protocol_state")
            ],
            program.programId
        );

        // const _tx = await program.rpc.initialize({
        //     accounts: {
        //         authority: environmentWallet.publicKey,
        //         protocolSigner,
        //         protocolState,
        //         rent: SYSVAR_RENT_PUBKEY,
        //         systemProgram: SystemProgram.programId
        //     }
        // })
    });

    it('Subscription plan - Invalid amount, Invalid frequency', async () => {
        const subscriptionPlanName = "Plan A";
        
        const [subscriptionPlanAuthor, subscriptionPlanAuthorBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("subscription_plan_author"),
                subscriptionPlanAuthorWallet.publicKey.toBuffer(),
            ],
            program.programId
        );

        const [subscriptionPlan, subscriptionPlanBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("subscription_plan"),
                utf8.encode(subscriptionPlanName),
                subscriptionPlanAuthor.toBuffer(),
            ],
            program.programId
        );


        const createInvalidPlan = async (amount: number, frequency: number, fee_percentage: number) => {
            try {
                const _tx = await program.rpc.createSubscriptionPlan(subscriptionPlanName, new BN(amount), new BN(frequency), new BN(fee_percentage), {
                    accounts: {
                        protocolState,
                        subscriptionPlanAuthor,
                        authority: subscriptionPlanAuthorWallet.publicKey,
                        mint,
                        subscriptionPlanPaymentAccount: subscriptionPaymentUSDCAssociatedAccount,
                        rent: SYSVAR_RENT_PUBKEY,
                        subscriptionPlan,
                        tokenProgram: TOKEN_PROGRAM_ID,
                        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                        systemProgram: SystemProgram.programId
                    },
                    signers: [subscriptionPlanAuthorWallet.payer]
                });
                return null;
            } catch (e) {
                return e;
            }
        }

        // invalid amount min
        let error1 = await createInvalidPlan(0.5 * Math.pow(10, mint_decimals), 60, 3);
        assert.ok(
            error1 != null && error1.code === 6008,
            "Invalid amount check failed."
        );
        
        // invalid amount max
        let error2 = await createInvalidPlan(100000 * Math.pow(10, mint_decimals), 60, 1);
        assert.ok(
            error2 != null && error1.code === 6008,
            "Invalid amount check failed."
        );

        // invalid frequency
        let error3 = await createInvalidPlan(20 * Math.pow(10, mint_decimals), 10, 2);
        assert.ok(
            error3 != null && error3.code === 6011,
            "Invalid frequency check failed."
        );

        // invalid fee
        let error4 = await createInvalidPlan(20 * Math.pow(10, mint_decimals), 60, 0.5);
        assert.ok(
            error4 != null && error4.code === 6014,
            "Invalid fee check failed."
        );

         // invalid fee
         let error5 = await createInvalidPlan(20 * Math.pow(10, mint_decimals), 60, 7);
         assert.ok(
              error5 != null && error5.code === 6014,
             "Invalid fee check failed."
         );
    });
});