use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{token::{Mint, TokenAccount, Token}, associated_token::AssociatedToken};

#[derive(Accounts)]
#[instruction(plan_name: String)]
pub struct CreateSubscriptionPlan<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump
    )]
    pub protocol_state: Box<Account<'info, Protocol>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"subscription_plan_author", authority.key().as_ref()],
        bump,
        space = 8 + 10000 // todo: calculate correct space
    )]
    pub subscription_plan_author: Box<Account<'info, SubscriptionPlanAuthor>>,

    #[account(
        init,
        payer = authority,
        seeds = [b"subscription_plan", plan_name.as_bytes(), authority.key().as_ref()],
        bump,
        space = 8 + 10000 // todo: calculate correct space
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
    subscription_plan.subscription_plan_payment_account = ctx.accounts.subscription_plan_payment_account.key();
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

    subscription_plan.subscription_accounts = vec![];
    subscription_plan_author
        .subscription_plan_accounts
        .push(subscription_plan.key());

    let state = &mut ctx.accounts.protocol_state;
    state.subscription_plan_accounts.push(subscription_plan.key());
    
    Ok(())
}
