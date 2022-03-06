use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Unsubscribe<'info> {
    #[account(mut)]
    pub who_unsubscribes: Signer<'info>,

    #[account(
        mut,
        seeds = [b"subscription", subscriber.key().as_ref(), subscription_plan.key().as_ref()],
        bump = subscription.bump,
        has_one = subscriber,
        has_one = subscription_plan,
        constraint = subscription.has_already_been_initialized @ ErrorCode::SubscriptionNotInitialized,
        constraint = subscription.is_active @ ErrorCode::SubscriptionNotSubscribed,
    )]
    pub subscription: Box<Account<'info, Subscription>>,

    #[account(
        mut,
        seeds = [b"state", who_unsubscribes.key().as_ref()],
        bump = subscriber.bump,
        constraint = subscriber.has_already_been_initialized @ ErrorCode::SubscriberNotInitialized,
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,

    #[account(
        mut,
        constraint = subscription_plan.has_already_been_initialized @ ErrorCode::SubscriptionPlanNotInitialized
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,
}

pub fn handler(ctx: Context<Unsubscribe>) -> Result<()> {
    let subscription = &mut ctx.accounts.subscription;

    // cancel subscription
    subscription.is_active = false;
    subscription.is_cancelled = true;

    Ok(())
}
