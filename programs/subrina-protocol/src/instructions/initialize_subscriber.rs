use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct InitializeSubscriberAccount<'info> {
    #[account(mut)]
    pub who_subscribes: Signer<'info>,

    #[account(
        init,
        payer = who_subscribes,
        seeds = [b"state", who_subscribes.key().as_ref()],
        bump,
        space=8+1000 //todo: calculate correct space
    )]
    pub subscriber: Box<Account<'info, Subscriber>>,

    #[account(
        init_if_needed,
        payer = who_subscribes,
        associated_token::mint = mint,
        associated_token::authority = who_subscribes,
    )]
    pub subscriber_token_account: Box<Account<'info, TokenAccount>>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)] // remove on testing
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeSubscriberAccount>) -> Result<()> {
    let subscriber = &mut ctx.accounts.subscriber;
    subscriber.bump = *ctx.bumps.get("subscriber").unwrap();
    subscriber.has_already_been_initialized = true;
    subscriber.authority = ctx.accounts.who_subscribes.key();
    subscriber.subscriber_payment_account = ctx.accounts.subscriber_token_account.key();
    
    Ok(())
}
