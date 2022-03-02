use crate::{error::ErrorCode, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    mint,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct RegisterNode<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"node", authority.key().as_ref()],
        bump,
        space=8+1000 //todo: calculate correct space
    )]
    pub node: Box<Account<'info, Node>>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint,
        associated_token::authority = node_payment_wallet,
    )]
    pub node_payment_account: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"protocol_state"],
        bump = protocol_state.bump,
        constraint = protocol_state.has_already_been_initialized
    )]
    pub protocol_state: Box<Account<'info, Protocol>>,

    /// CHECK: authority is checked before changing payment wallet and mint account
    pub node_payment_wallet: UncheckedAccount<'info>,

    // #[account(address = mint::USDC @ ErrorCode::InvalidMint)] // remove on testing
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<RegisterNode>) -> Result<()> {
    let node = &mut ctx.accounts.node;
    
    if !node.is_registered {
        node.is_registered = true;
        node.bump = *ctx.bumps.get("node").unwrap();
        node.authority = ctx.accounts.authority.key();
        node.node_payment_wallet = ctx.accounts.node_payment_wallet.key();
        node.node_payment_account = ctx.accounts.node_payment_account.key();

        let protocol_state = &mut ctx.accounts.protocol_state;
        protocol_state.registered_nodes.push(node.key())
    } else {
        require!(
            node.authority.eq(&ctx.accounts.authority.key()),
            ErrorCode::NodeErrorUnauthorized
        );
        node.node_payment_wallet = ctx.accounts.node_payment_wallet.key();
        node.node_payment_account = ctx.accounts.node_payment_account.key();
    }
    Ok(())
}
