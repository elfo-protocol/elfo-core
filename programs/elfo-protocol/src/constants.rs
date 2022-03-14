use anchor_lang::prelude::*;


// cancellation reasons
pub const CANCELLATION_INSUFFICIENT_AMOUNT: i8 = 1;
pub const CANCELLATION_DELEGATION_REVOKED: i8 = 2;
pub const CANCELLATION_DELEGATED_AMOUNT_NOT_ENOUGH: i8 = 3;

pub const PUBKEY_SIZE: usize = std::mem::size_of::<Pubkey>();

// these constants needs to be changed when protocol gets bigger
pub const MAXIMUM_SUBSCRIPTIONS_PER_PLAN :usize = 10;
pub const MAXIMUM_SUBSCRIPTIONS_PER_USER :usize = 20;
pub const MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR: usize = 10;
pub const MAXIMUM_SUBSCRIPTION_PLANS: usize = 100;
pub const MAXIMUM_NODES: usize = 50;