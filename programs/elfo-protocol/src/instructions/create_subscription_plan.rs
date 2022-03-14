use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(plan_name: String)]
pub struct CreateSubscriptionPlan<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump,
        constraint = protocol_state.has_already_been_initialized
    )]
    pub protocol_state: Box<Account<'info, Protocol>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"subscription_plan_author", authority.key().as_ref()],
        bump,
        space = SubscriptionPlanAuthor::space()
    )]
    pub subscription_plan_author: Box<Account<'info, SubscriptionPlanAuthor>>,

    #[account(
        init,
        payer = authority,
        seeds = [b"subscription_plan", plan_name.as_bytes(), subscription_plan_author.key().as_ref()],
        bump,
        space = SubscriptionPlan::space(&plan_name)
    )]
    pub subscription_plan: Box<Account<'info, SubscriptionPlan>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = authority,
    )]
    pub subscription_plan_payment_account: Box<Account<'info, TokenAccount>>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(
    ctx: Context<CreateSubscriptionPlan>,
    plan_name: String,
    subscription_amount: i64,
    frequency: i64,
    fee_percentage: i8,
) -> Result<()> {
    let subscription_plan_author = &mut ctx.accounts.subscription_plan_author;

    if !subscription_plan_author.has_already_been_initialized {
        subscription_plan_author.has_already_been_initialized = true;
        subscription_plan_author.authority = ctx.accounts.authority.key();
        subscription_plan_author.bump = *ctx.bumps.get("subscription_plan_author").unwrap();
        subscription_plan_author.subscription_plan_accounts = vec![];
    }

    let subscription_plan = &mut ctx.accounts.subscription_plan;
    subscription_plan.has_already_been_initialized = true;
    subscription_plan.bump = *ctx.bumps.get("subscription_plan").unwrap();
    subscription_plan.plan_name = plan_name;
    subscription_plan.subscription_plan_author = subscription_plan_author.key();
    subscription_plan.subscription_plan_payment_account =
        ctx.accounts.subscription_plan_payment_account.key();
    subscription_plan.is_active = true;

    let multiplier: i64 = 10_i32.pow(ctx.accounts.mint.decimals.into()).into();
    require!(
        subscription_amount > 1 * multiplier,
        ErrorCode::SubscriptionPlanAmountInvalid
    );
    require!(
        subscription_amount < 1001 * multiplier,
        ErrorCode::SubscriptionPlanAmountInvalid
    );
    subscription_plan.amount = subscription_amount;

    require!(frequency >= 60, ErrorCode::SubscriptionPlanFrequencyError);
    subscription_plan.frequency = frequency;

    require!(fee_percentage >= 1, ErrorCode::SubscriptionPlanFeeError);
    require!(fee_percentage <= 5, ErrorCode::SubscriptionPlanFeeError);
    subscription_plan.fee_percentage = fee_percentage;

    subscription_plan.subscription_accounts = vec![];
    subscription_plan_author
        .subscription_plan_accounts
        .push(subscription_plan.key());

    let state = &mut ctx.accounts.protocol_state;
    state
        .subscription_plan_accounts
        .push(subscription_plan.key());

    Ok(())
}
