use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CloseSubscriptionPlan<'info> {
    #[account(
        seeds = [b"subscription_plan_author", authority.key().as_ref()],
        has_one = authority @ErrorCode::SubscriptionPlanUnauthorizedToClose,
        bump = subscription_plan_author.bump,
    )]
    pub subscription_plan_author: Box<Account<'info, SubscriptionPlanAuthor>>,

    #[account(
        mut,
        has_one = subscription_plan_author @ErrorCode::SubscriptionPlanUnauthorizedToClose,
        constraint = subscription_plan.has_already_been_initialized @ ErrorCode::SubscriptionPlanNotInitialized,
        constraint = subscription_plan.is_active @ ErrorCode::SubscriptionPlanAlreadyClosed
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<CloseSubscriptionPlan>) -> Result<()> {
    let subscription_plan = &mut ctx.accounts.subscription_plan;
    subscription_plan.is_active = false;
    Ok(())
}
