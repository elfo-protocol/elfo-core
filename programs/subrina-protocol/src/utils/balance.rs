use std::convert::TryInto;

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

use crate::state::*;

pub fn has_enough_balance(
    account_from: &Box<Account<TokenAccount>>,
    subscription_plan: &Box<Account<SubscriptionPlan>>,
) -> Result<bool> {
    let balance_of_user = token::accessor::amount(&account_from.to_account_info())?;
    let required_balance = subscription_plan.amount;

    if balance_of_user >= required_balance.try_into().unwrap() {
        return Ok(true);
    }
    Ok(false)
}

pub fn charge_for_one_cycle<'info>(
    protocol_signer: &Box<Account<'info, ProtocolSigner>>,
    subscriber_token_wallet: &Box<Account<'info, TokenAccount>>,
    payment_account: &Box<Account<'info, TokenAccount>>,
    subscription_plan: &Box<Account<'info, SubscriptionPlan>>,
    token_program: &Program<'info, Token>,
) -> Result<()> {
    let bump = vec![protocol_signer.bump];
    let inner_seeds = vec![
        b"protocol_signer".as_ref(),
        bump.as_ref(),
    ];
    let signer_seeds = vec![&inner_seeds[..]];

    anchor_spl::token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            anchor_spl::token::Transfer {
                from: subscriber_token_wallet.to_account_info().clone(),
                to: payment_account.to_account_info(),
                authority: protocol_signer.to_account_info().clone(),
            },
            &signer_seeds,
        ),
        subscription_plan.amount as u64,
    )?;
    Ok(())
}