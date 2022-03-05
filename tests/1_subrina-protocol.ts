import * as anchor from '@project-serum/anchor';
import { Program, Provider, BN, Wallet } from '@project-serum/anchor';
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAccount } from '@solana/spl-token';

import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    SystemProgram,
    SYSVAR_RENT_PUBKEY,
    SYSVAR_CLOCK_PUBKEY
} from '@solana/web3.js';
import { assert } from 'chai';
import { SubrinaProtocol } from '../target/types/subrina_protocol';
import { delay } from './utils/timer';
import { createMint, createAssocciatedTokenAccount, mintUSDC } from './utils/token';

const utf8 = anchor.utils.bytes.utf8;

describe('[subrina-protocol] - Positive Test Cases', () => {

    const provider = Provider.env();
    anchor.setProvider(provider);

    const program = anchor.workspace.SubrinaProtocol as Program<SubrinaProtocol>;
    const environmentWallet = program.provider.wallet;

    let mint: PublicKey, mint_decimals = 6;
    let subscriptionPlanAuthorWallet: Wallet, subscriptionPaymentUSDCAssociatedAccount: PublicKey;
    let subscriberWallet: Wallet, subscriberUSDCAssociatedAccount: PublicKey;
    let nodeAuthorityWallet: Wallet, nodePaymentWallet: Wallet, nodeUSDCAssociatedAccount: PublicKey;

    let protocolState: PublicKey, protocolSigner: PublicKey;

    let subscriptionPlanName: string,
        subscriptionPlan: PublicKey,
        subscriptionPlanAuthor: PublicKey,
        subscriber: PublicKey,
        subscription: PublicKey,
        node: PublicKey,
        subscriptionPlanAmount = 20 * Math.pow(10, mint_decimals),
        frequency = 90,
        how_many_cycles = 12,
        fee_percentage = 5;

    let protocolSignerBump: number,
        protocolStateBump: number,
        subscriptionPlanBump: number,
        subscriptionPlanAuthorBump: number,
        subscriberBump: number,
        subscriptionBump: number,
        nodeBump;

    before('Boilerplates', async () => {
        // Creating a wallet for subscription author
        subscriptionPlanAuthorWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(subscriptionPlanAuthorWallet.publicKey, 1000 * LAMPORTS_PER_SOL);

        // Creating wallets for subscribers
        subscriberWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(subscriberWallet.publicKey, 1000 * LAMPORTS_PER_SOL);

        // Create wallet for a node
        nodeAuthorityWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(nodeAuthorityWallet.publicKey, 1000 * LAMPORTS_PER_SOL);
        nodePaymentWallet = new Wallet(Keypair.generate());
        await provider.connection.requestAirdrop(nodePaymentWallet.publicKey, 1000 * LAMPORTS_PER_SOL);

        // Creating a dummy USDC mint
        mint = await createMint(provider, environmentWallet.publicKey, 6);

        // Creating subscription author payment account with 5000 USDC
        subscriptionPaymentUSDCAssociatedAccount = await createAssocciatedTokenAccount(provider, mint, subscriptionPlanAuthorWallet.publicKey);
        await mintUSDC(provider, mint, subscriptionPaymentUSDCAssociatedAccount, environmentWallet.publicKey, 5000 * Math.pow(10, mint_decimals))

        // Createing subscriber token payment account with 5000 USDC
        subscriberUSDCAssociatedAccount = await createAssocciatedTokenAccount(provider, mint, subscriberWallet.publicKey);
        await mintUSDC(provider, mint, subscriberUSDCAssociatedAccount, environmentWallet.publicKey, 5000 * Math.pow(10, mint_decimals))

        // Createing a node token payment account with 5000 USDC
        nodeUSDCAssociatedAccount = await createAssocciatedTokenAccount(provider, mint, nodePaymentWallet.publicKey);
        await mintUSDC(provider, mint, nodeUSDCAssociatedAccount, environmentWallet.publicKey, 5000 * Math.pow(10, mint_decimals))
    });

    it('Initialize protocol', async () => {

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
        const _tx = await program.rpc.initialize({
            accounts: {
                authority: environmentWallet.publicKey,
                protocolSigner,
                protocolState,
                rent: SYSVAR_RENT_PUBKEY,
                systemProgram: SystemProgram.programId
            }
        })
    });

    it('Verifies protocol details', async () => {
        const protocolSignerAccount = await program.account.protocolSigner.fetch(protocolSigner);
        assert.ok(
            protocolSignerAccount.bump = protocolSignerBump,
            "Incrorrect pda bump."
        );

        const protocolStateAccount = await program.account.protocol.fetch(protocolState);

        assert.ok(
            protocolStateAccount.hasAlreadyBeenInitialized,
            "Not initialized."
        );

        assert.ok(
            protocolStateAccount.bump = protocolStateBump,
            "Incrorrect pda bump."
        );

        assert.ok(
            protocolStateAccount.authority = environmentWallet.publicKey,
            "Authority not set."
        );

        assert.ok(
            protocolStateAccount.subscriptionPlanAccounts.length == 0,
            "Plan account list not set."
        );

        assert.ok(
            protocolStateAccount.registeredNodes.length == 0,
            "Node account list not set."
        );

    });

    it('Creates a subscription plan', async () => {

        subscriptionPlanName = "Plan A";

        [subscriptionPlanAuthor, subscriptionPlanAuthorBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("subscription_plan_author"),
                subscriptionPlanAuthorWallet.publicKey.toBuffer(),
            ],
            program.programId
        );

        [subscriptionPlan, subscriptionPlanBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("subscription_plan"),
                utf8.encode(subscriptionPlanName),
                subscriptionPlanAuthor.toBuffer(),
            ],
            program.programId
        );

        const _tx = await program.rpc.createSubscriptionPlan(subscriptionPlanName, new BN(subscriptionPlanAmount), new BN(frequency), new BN(fee_percentage), {
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
    });

    it('Verifies subscription plan author details', async () => {
        const subscriptionPlanAuthorAccount = await program.account.subscriptionPlanAuthor.fetch(subscriptionPlanAuthor);

        assert.ok(
            subscriptionPlanAuthorAccount.bump = subscriptionPlanAuthorBump,
            "Incrorrect pda bump."
        );

        assert.ok(
            subscriptionPlanAuthorAccount.hasAlreadyBeenInitialized == true,
            "Not initialzed."
        );

        assert.ok(
            subscriptionPlanAuthorAccount.authority.equals(subscriptionPlanAuthorWallet.publicKey),
            "Incorrect authority."
        );
        assert.ok(
            subscriptionPlanAuthorAccount.subscriptionPlanAccounts.at(0).equals(subscriptionPlan),
            "Subscription plan list does not include created plan."
        );
    });

    it('Verifies subscription plan details', async () => {
        const subscriptionPlanAccount = await program.account.subscriptionPlan.fetch(subscriptionPlan);
        assert.ok(
            subscriptionPlanAccount.bump = subscriptionPlanBump,
            "Incrorrect pda bump."
        );

        assert.ok(
            subscriptionPlanAccount.hasAlreadyBeenInitialized == true,
            "Not initialzed."
        );

        assert.ok(
            subscriptionPlanAccount.planName === subscriptionPlanName,
            "Incorrect plan name."
        );

        assert.ok(
            subscriptionPlanAccount.subscriptionPlanAuthor.equals(subscriptionPlanAuthor),
            "Incorrect author."
        );

        assert.ok(
            subscriptionPlanAccount.subscriptionPlanPaymentAccount.equals(subscriptionPaymentUSDCAssociatedAccount),
            "Incorrect payment account."
        );

        assert.ok(
            subscriptionPlanAccount.amount.eq(new BN(subscriptionPlanAmount)),
            "Incorrect amount."
        );

        assert.ok(
            subscriptionPlanAccount.frequency.eq(new BN(frequency)),
            "Incorrect frequency."
        );

        assert.ok(
            subscriptionPlanAccount.isActive == true,
            "Incorrect active status."
        );

        assert.ok(
            subscriptionPlanAccount.subscriptionAccounts.length == 0,
            "Subscription account list is not empty."
        );

        const protocolStateAccount = await program.account.protocol.fetch(protocolState);
        assert.ok(
            protocolStateAccount.subscriptionPlanAccounts.at(0).equals(subscriptionPlan),
            "Protocol state is not updated."
        );
    });

    it('Subscribes to a plan', async () => {
        const balanceBefore = parseInt((await provider.connection.getTokenAccountBalance(subscriberUSDCAssociatedAccount)).value.amount);
        const balanceBeforePaymentlWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriptionPaymentUSDCAssociatedAccount)).value.amount);

        [subscriber, subscriberBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("state"),
                subscriberWallet.publicKey.toBuffer(),
            ],
            program.programId
        );

        [subscription, subscriptionBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("subscription"),
                subscriber.toBuffer(),
                subscriptionPlan.toBuffer()
            ],
            program.programId
        );

        const _tx = await program.rpc.subscribe(new BN(how_many_cycles), {
            accounts: {
                subscription,
                protocolSigner,
                subscriberPaymentAccount: subscriberUSDCAssociatedAccount,
                whoSubscribes: subscriberWallet.publicKey,
                subscriptionPlan,
                subscriber,
                subscriptionPlanPaymentAccount: subscriptionPaymentUSDCAssociatedAccount,
                mint,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                clock: SYSVAR_CLOCK_PUBKEY,
                rent: SYSVAR_RENT_PUBKEY,

                systemProgram: SystemProgram.programId
            },
            signers: [subscriberWallet.payer]
        });

        const balanceAfter = parseInt((await provider.connection.getTokenAccountBalance(subscriberUSDCAssociatedAccount)).value.amount);
        const balanceAfterPaymentlWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriptionPaymentUSDCAssociatedAccount)).value.amount);

        assert.ok(balanceBefore - balanceAfter == subscriptionPlanAmount,
            "Subscription amount not charged properly.");

        assert.ok(balanceAfterPaymentlWallet - balanceBeforePaymentlWallet == subscriptionPlanAmount,
            "Subscription amount not charged properly.");
    });

    it('Verifies subscriber account details', async () => {
        const subscriberAccount = await program.account.subscriber.fetch(subscriber);

        assert.ok(
            subscriberAccount.bump = subscriberBump,
            "Incrorrect pda bump."
        );

        assert.ok(
            subscriberAccount.subscriberPaymentAccount.equals(subscriberUSDCAssociatedAccount),
            "Incrorrect token account assigned to subscriber."
        );

        assert.ok(
            subscriberAccount.hasAlreadyBeenInitialized,
            "Not initialized."
        );

    });

    it('Verifies delgation', async () => {
        const delegatedAccount = await getAccount(provider.connection, subscriberUSDCAssociatedAccount);

        assert.ok(
            delegatedAccount.delegate.equals(protocolSigner),
            "Account not properly delegated."
        );

        assert.ok(
            BigInt(delegatedAccount.delegatedAmount) === BigInt(subscriptionPlanAmount * (how_many_cycles - 1)),
            "Delegated amount is"
        )
    })

    it('Verifies subscription details', async () => {
        const subscriptionPlanAccount = await program.account.subscriptionPlan.fetch(subscriptionPlan);
        const subscriberAccount = await program.account.subscriber.fetch(subscriber);
        const subscriptionAccount = await program.account.subscription.fetch(subscription);

        assert.ok(
            subscriptionAccount.bump = subscriptionBump,
            "Incrorrect pda bump."
        );

        assert.ok(
            subscriptionAccount.hasAlreadyBeenInitialized,
            "Not initialized."
        );

        assert.ok(
            subscriptionAccount.subscriber.equals(subscriber),
            "Incorrect subscriber."
        );

        assert.ok(
            subscriptionAccount.subscriptionPlan.equals(subscriptionPlan),
            "Incorrect subscription plan."
        );

        assert.ok(
            subscriptionAccount.isActive,
            "Subscription account is inactive."
        );

        assert.ok(
            subscriptionAccount.isCancelled == false,
            "Subscription cancellation status is incorrect."
        );

        const timestampNow = Math.round((+new Date()) / 1000);
        const lastTimestamp = subscriptionAccount.lastPaymentTimestamp.toNumber();
        assert.ok(Math.abs(timestampNow - lastTimestamp) < 5, "Last payment timestamp doesn't match.");

        const nextTimestamp = subscriptionAccount.nextPaymentTimestamp.toNumber();
        const nextTimetampExpected = timestampNow + subscriptionPlanAccount.frequency.toNumber();
        assert.ok(Math.abs(nextTimestamp - nextTimetampExpected) < 5, "Next payment timestamp doesn't match.");

        assert.ok(subscriptionPlanAccount.subscriptionAccounts.at(0).equals(subscription),
            "Subcription plan account does not contain subscription.");

        assert.ok(subscriberAccount.subscriptionAccounts.at(0).equals(subscription),
            "Subcriber account does not contain the subscription.")
    });

    it('Register a node.', async () => {
        [node, nodeBump] = await PublicKey.findProgramAddress(
            [
                utf8.encode("node"),
                nodeAuthorityWallet.publicKey.toBuffer()
            ],
            program.programId
        );

        const _tx = await program.rpc.registerNode({
            accounts: {
                protocolState,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                authority: nodeAuthorityWallet.publicKey,
                mint,
                node,
                nodePaymentAccount: nodeUSDCAssociatedAccount,
                nodePaymentWallet: nodePaymentWallet.publicKey,
                rent: SYSVAR_RENT_PUBKEY,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID
            },
            signers: [nodeAuthorityWallet.payer]
        })
    });

    it('Verifies node details.', async () => {
        const nodeAccount = await program.account.node.fetch(node);
        const protocolAccount = await program.account.protocol.fetch(protocolState);
        assert.ok(
            nodeAccount.bump == nodeBump,
            "Pda not set properly"
        );

        assert.ok(
            nodeAccount.isRegistered,
            "Node not registered."
        );

        assert.ok(
            nodeAccount.authority.equals(nodeAuthorityWallet.publicKey),
            "Authority wallet not set."
        );

        assert.ok(
            nodeAccount.nodePaymentWallet.equals(nodePaymentWallet.publicKey),
            "Payment wallet not set."
        );

        assert.ok(
            nodeAccount.nodePaymentAccount.equals(nodeUSDCAssociatedAccount),
            "Node payment account not set."
        );

        assert.ok(
            protocolAccount.registeredNodes.at(0).equals(node),
            "Node not is protocol state."
        )
    });

    it('Trigger payment of subscription.', async () => {
        const balanceBeforeSubscriberWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriberUSDCAssociatedAccount)).value.amount);
        const balanceBeforePaymentlWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriptionPaymentUSDCAssociatedAccount)).value.amount);
        const balanceBeforeNodeWallet = parseInt((await provider.connection.getTokenAccountBalance(nodeUSDCAssociatedAccount)).value.amount)
        
        const waitTime = frequency + 10;
        console.log(`Waiting ${waitTime} seconds before trying taking payment.`);
        await delay(waitTime * 1000);

        const _tx = await program.rpc.triggerPayment({
            accounts: {
                node,
                nodePaymentAccount: nodeUSDCAssociatedAccount,
                authority: nodeAuthorityWallet.publicKey,
                nodePaymentWallet: nodePaymentWallet.publicKey,
                protocolSigner,
                subscriberPaymentAccount: subscriberUSDCAssociatedAccount,
                subscriber,
                subscription,
                subscriptionPlan,
                subscriptionPlanPaymentAccount: subscriptionPaymentUSDCAssociatedAccount,
                tokenProgram: TOKEN_PROGRAM_ID,
                clock: SYSVAR_CLOCK_PUBKEY,
                mint
            },
            signers: [nodeAuthorityWallet.payer]
        });

        const balanceAfterSubscriberWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriberUSDCAssociatedAccount)).value.amount);
        const balanceAfterPaymentlWallet = parseInt((await provider.connection.getTokenAccountBalance(subscriptionPaymentUSDCAssociatedAccount)).value.amount);
        const balanceAfterNodeWallet = parseInt((await provider.connection.getTokenAccountBalance(nodeUSDCAssociatedAccount)).value.amount);

        assert.ok(balanceBeforeSubscriberWallet - balanceAfterSubscriberWallet == subscriptionPlanAmount,
            "Subscription amount not charged properly.");

        const fee = subscriptionPlanAmount * fee_percentage/100;
        assert.ok(balanceAfterPaymentlWallet - balanceBeforePaymentlWallet == subscriptionPlanAmount - fee,
            "Subscription amount not charged properly.");

        assert.ok(balanceAfterNodeWallet - balanceBeforeNodeWallet == fee,
                "Subscription amount not charged properly.");

        const subscriptionAccount = await program.account.subscription.fetch(subscription);
        const subscriptionPlanAccount = await program.account.subscriptionPlan.fetch(subscriptionPlan);

        const timestampNow = Math.round((+new Date()) / 1000);
        const lastTimestamp = subscriptionAccount.lastPaymentTimestamp.toNumber();
        assert.ok(Math.abs(timestampNow - lastTimestamp) < 5, "Last payment timestamp doesn't match.");

        const nextTimestamp = subscriptionAccount.nextPaymentTimestamp.toNumber();
        const nextTimetampExpected = timestampNow + subscriptionPlanAccount.frequency.toNumber();
        assert.ok(Math.abs(nextTimestamp - nextTimetampExpected) < 5, "Next payment timestamp doesn't match.");
    });

    it('Closes the subscription.', async () => {
        const _tx = await program.rpc.closeSubscriptionPlan({
            accounts: {
                authority: subscriptionPlanAuthorWallet.publicKey,
                subscriptionPlan,
                subscriptionPlanAuthor
            },
            signers: [
                subscriptionPlanAuthorWallet.payer
            ]
        });
        const subscriptionPlanAccount = await program.account.subscriptionPlan.fetch(subscriptionPlan);
        assert.ok(
            subscriptionPlanAccount.isActive == false,
            "Subscription plan is not closed."
        )
    });

    it('Unsubscribes.', async () => {
        const _tx = await program.rpc.unsubscribe({
            accounts: {
                subscriber,
                subscription,
                whoUnsubscribes: subscriberWallet.publicKey,
                subscriptionPlan
            },
            signers: [
                subscriberWallet.payer
            ]
        });
        const subscriptionAccount = await program.account.subscription.fetch(subscription);
        assert.ok(
            subscriptionAccount.isActive == false,
            "Subscription is not closed."
        );

        assert.ok(
            subscriptionAccount.isCancelled,
            "Subscription is not cancelled.."
        )
    });
});
