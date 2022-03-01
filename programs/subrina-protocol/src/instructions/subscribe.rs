use std::convert::TryInto;

use crate::{error::ErrorCode, state::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    mint,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Subscribe<'info> {
    #[account(mut)]
    pub who_subscribes: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol_signer"],
        bump = protocol_signer.bump,
    )]
    pub protocol_signer: Box<Account<'info, ProtocolSigner>>,

    #[account(
        init_if_needed,
        payer = who_subscribes,
        seeds = [b"subscription", subscriber.key().as_ref(), subscription_plan.key().as_ref()],
        bump,
        space=8+1000 //todo: calculate correct space
    )]
    pub subscription: Box<Account<'info, Subscription>>,

    #[account(
        mut,
        seeds = [b"state", who_subscribes.key().as_ref()],
        bump = subscriber.bump,
        has_one = subscriber_payment_account,
        constraint = subscriber.has_already_been_initialized @ ErrorCode::SubscriberNotInitialized,
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = who_subscribes,
    )]
    pub subscriber_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = subscription_plan.has_already_been_initialized @ ErrorCode::SubscriptionPlanNotInitialized,
        constraint = subscription_plan.is_active @ ErrorCode::SubscriptionPlanInactive,
        has_one = subscription_plan_payment_account @ErrorCode::SubscriptionPlanInvalidPaymentAccount
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    #[account(
        mut,
        constraint = subscription_plan_payment_account.mint ==  mint.key() @ ErrorCode::InvalidMint
    )]
    pub subscription_plan_payment_account: Box<Account<'info, TokenAccount>>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<Subscribe>, how_many_cycles: i64) -> Result<()> {
    // check if already subscribed
    let subscription = &mut ctx.accounts.subscription;
    let subscriber = &mut ctx.accounts.subscriber;
    let subscription_plan = &mut ctx.accounts.subscription_plan;
    let subscriber_token_wallet = &mut ctx.accounts.subscriber_payment_account;

    if subscription.has_already_been_initialized {
        // user has already been interracted with this subscription before
        require!(
            subscription.is_active,
            ErrorCode::SubscriptionAlreadySubscribed
        );
    } else {
        subscription.has_already_been_initialized = true;
        subscription.bump = *ctx.bumps.get("subscription").unwrap();
        subscription.subscriber = subscriber.key();
        subscription.subscription_plan = subscription_plan.key();

        subscriber
            .subscription_accounts
            .push(subscription.key().clone());
        subscription_plan
            .subscription_accounts
            .push(subscription.key().clone());
    }

    // check if the subscriber has enough funds for the first cycle
    require!(
        has_enough_balance(&subscriber_token_wallet, subscription_plan)?,
        ErrorCode::SubscriptionNotEnoughFunds
    );

    // check for delegation
    let mut amount_to_delegate: i64 = subscription_plan.amount * how_many_cycles;
    match subscriber_token_wallet.delegate {
        anchor_lang::solana_program::program_option::COption::None => {
        },
        anchor_lang::solana_program::program_option::COption::Some(delegated_account) => {
            if delegated_account.eq(&ctx.accounts.protocol_signer.key()) {
                let increment: i64 = subscriber_token_wallet.delegated_amount.try_into().unwrap();
                amount_to_delegate = amount_to_delegate + increment;
            }
        },
    }
    
    anchor_spl::token::approve(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
             anchor_spl::token::Approve { 
                 delegate: ctx.accounts.protocol_signer.to_account_info(),
                 to: subscriber_token_wallet.to_account_info(),
                 authority: ctx.accounts.who_subscribes.to_account_info()
                 }
            ),
            amount_to_delegate.try_into().unwrap(),
    )?;

    charge_for_one_cycle(
        &ctx.accounts.protocol_signer,
        &ctx.accounts.subscriber_payment_account,
        &ctx.accounts.subscription_plan_payment_account,
        &subscription_plan,
        &ctx.accounts.token_program,
    )?;

    subscription.is_active = true;

    let clock = &ctx.accounts.clock;
    let current_time = clock.unix_timestamp;

    subscription.last_payment_timestamp = current_time;
    subscription.next_payment_timestamp = current_time + subscription_plan.frequency;

    Ok(())
}
