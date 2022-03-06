use crate::state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeProtocol<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [b"protocol_signer"],
        bump,
        space =8+100 // todo: calculate correct space
    )]
    pub protocol_signer: Box<Account<'info, ProtocolSigner>>,

    #[account(
        init,
        payer = authority,
        seeds = [b"protocol_state"],
        bump,
        space=8+1000 //todo: calculate correct space
    )]
    pub protocol_state: Box<Account<'info, Protocol>>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<InitializeProtocol>) -> Result<()> {
    let protocol_signer = &mut ctx.accounts.protocol_signer;
    protocol_signer.bump = *ctx.bumps.get("protocol_signer").unwrap();

    let protocol_state = &mut ctx.accounts.protocol_state;
    protocol_state.bump = *ctx.bumps.get("protocol_state").unwrap();
    protocol_state.has_already_been_initialized = true;
    protocol_state.authority = ctx.accounts.authority.key();
    protocol_state.subscription_plan_accounts = vec![];
    protocol_state.registered_nodes = vec![];
    Ok(())
}
