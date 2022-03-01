use crate::{error::ErrorCode, state::*, utils::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    mint,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TakePayment<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = subscriber_payment_account.mint ==  mint.key() @ ErrorCode::InvalidMint
    )]
    pub subscriber_payment_account: Box<Account<'info, TokenAccount>>,


    #[account(
        mut,
        seeds = [b"protocol_signer"],
        bump = protocol_signer.bump
    )]
    pub protocol_signer: Box<Account<'info, ProtocolSigner>>,

    #[account(
        mut,
        has_one = subscription_plan,
        has_one = subscriber,
        constraint = subscription.has_already_been_initialized @ErrorCode::SubscriberNotInitialized,
        constraint = subscription.is_active @ ErrorCode::SubscriptionNotSubscribed,
    )]
    pub subscription: Box<Account<'info, Subscription>>,

    #[account(
        constraint = subscriber.has_already_been_initialized @ ErrorCode::SubscriberNotInitialized,
        has_one = subscriber_payment_account
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,

    #[account(
        constraint = subscription_plan.has_already_been_initialized @ErrorCode::SubscriptionPlanNotInitialized,
        constraint = subscription_plan.is_active @ ErrorCode::SubscriptionPlanInactive,
        has_one = subscription_plan_payment_account
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<TakePayment>) -> Result<()> {
    let subscription_plan = &ctx.accounts.subscription_plan;
    let subscription = &mut ctx.accounts.subscription;

    let clock = &ctx.accounts.clock;
    let current_time = clock.unix_timestamp;

    require!(
        subscription.next_payment_timestamp < current_time,
        ErrorCode::SubscriptionNextPaymentTimestampNotReached
    );

    if !has_enough_balance(&ctx.accounts.subscriber_payment_account, subscription_plan)? {
        // when all the conditions meet, but user has not enough funds
        // cancel the subscription
        subscription.is_active = false;
        subscription.is_cancelled = true;
        return Ok(());
    }

    charge_for_one_cycle(
        &ctx.accounts.protocol_signer,
        &ctx.accounts.subscriber_payment_account,
        &ctx.accounts.subscription_plan_payment_account,
        &subscription_plan,
        &ctx.accounts.token_program,
    )?;

    subscription.is_active = true;
    subscription.is_cancelled = false;

    subscription.last_payment_timestamp = current_time;
    subscription.next_payment_timestamp = current_time + subscription_plan.frequency;

    Ok(())
}
