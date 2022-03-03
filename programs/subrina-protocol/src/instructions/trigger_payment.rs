use std::convert::TryInto;

use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    mint,
    token::{self, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct TriggerPayment<'info> {
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
        mut,
        constraint = subscription_plan_payment_account.mint ==  mint.key() @ ErrorCode::InvalidMint
    )]
    pub subscription_plan_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        constraint = subscription_plan.has_already_been_initialized @ErrorCode::SubscriptionPlanNotInitialized,
        constraint = subscription_plan.is_active @ ErrorCode::SubscriptionPlanInactive,
        has_one = subscription_plan_payment_account
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    #[account(
        seeds = [b"node", authority.key().as_ref()],
        bump = node.bump,
        has_one = authority,
        has_one = node_payment_account,
        has_one = node_payment_wallet,
        constraint = node.is_registered @ErrorCode::NodeNotRegistered
    )]
    pub node: Box<Account<'info, Node>>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = node_payment_wallet,
    )]
    pub node_payment_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: node state is checked for payment wallet
    pub node_payment_wallet: UncheckedAccount<'info>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn handler(ctx: Context<TriggerPayment>) -> Result<()> {
    let subscription_plan = &ctx.accounts.subscription_plan;
    let subscription = &mut ctx.accounts.subscription;
    let subscriber_payment_account = &mut ctx.accounts.subscriber_payment_account;

    let clock = &ctx.accounts.clock;
    let current_time = clock.unix_timestamp;

    require!(
        subscription.next_payment_timestamp < current_time,
        ErrorCode::SubscriptionNextPaymentTimestampNotReached
    );

    let balance_of_user =
        token::accessor::amount(&subscriber_payment_account.to_account_info())?;
    let required_balance = subscription_plan.amount;

    let mut cancel_subscription = false;

    if !(balance_of_user >= required_balance.try_into().unwrap()) {
        // when all the conditions meet, but user has not enough funds
        // cancel the subscription
        cancel_subscription = true;
    }

    // check delegation
    match subscriber_payment_account.delegate {
        anchor_lang::solana_program::program_option::COption::None => {
            // no delegation
            // subscriber has revoked the delegation
            cancel_subscription = true;
        }
        anchor_lang::solana_program::program_option::COption::Some(delegated_account) => {
            if !delegated_account.eq(&ctx.accounts.protocol_signer.key()) {
                // delegated to wrong program
                cancel_subscription = true;
            }

            let delegated_amount: i64 = subscriber_payment_account
                    .delegated_amount
                    .try_into()
                    .unwrap();

            if delegated_amount < subscription_plan.amount {
                cancel_subscription = true;
            }
        }
    }

    if cancel_subscription {
        subscription.is_active = false;
        subscription.is_cancelled = true;
        return Ok(());
    }

    let percentage_for_node: i64 = subscription_plan.fee_percentage.into();
    let bump = vec![ctx.accounts.protocol_signer.bump];
    let inner_seeds = vec![b"protocol_signer".as_ref(), bump.as_ref()];
    let signer_seeds = vec![&inner_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx
                    .accounts
                    .subscriber_payment_account
                    .to_account_info()
                    .clone(),
                to: ctx
                    .accounts
                    .subscription_plan_payment_account
                    .to_account_info(),
                authority: ctx.accounts.protocol_signer.to_account_info().clone(),
            },
            &signer_seeds,
        ),
        (subscription_plan.amount * (100 - percentage_for_node) / 100) as u64,
    )?;

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: ctx
                    .accounts
                    .subscriber_payment_account
                    .to_account_info()
                    .clone(),
                to: ctx.accounts.node_payment_account.to_account_info(),
                authority: ctx.accounts.protocol_signer.to_account_info().clone(),
            },
            &signer_seeds,
        ),
        (subscription_plan.amount * (percentage_for_node) / 100) as u64,
    )?;

    subscription.is_active = true;
    subscription.is_cancelled = false;

    subscription.last_payment_timestamp = current_time;
    subscription.next_payment_timestamp = current_time + subscription_plan.frequency;

    Ok(())
}
