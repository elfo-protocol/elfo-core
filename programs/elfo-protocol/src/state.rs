use anchor_lang::prelude::*;

use crate::constants::{PUBKEY_SIZE, MAXIMUM_SUBSCRIPTIONS_PER_PLAN, MAXIMUM_SUBSCRIPTIONS_PER_USER, MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR, MAXIMUM_SUBSCRIPTION_PLANS, MAXIMUM_NODES};

#[account]
pub struct Subscriber {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,
    pub subscriber_payment_account: Pubkey,

    // This contains the PubKeys of all the subscription accounts the subscriber
    // has interacted with
    pub subscription_accounts: Vec<Pubkey>,
}
impl Subscriber {
    pub fn space() -> usize {
        8 +  // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // authority
        PUBKEY_SIZE + // subscriber_payment_account
        4 + (PUBKEY_SIZE * MAXIMUM_SUBSCRIPTIONS_PER_USER) //subscription_accounts
    }
}


#[account]
pub struct Subscription {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub subscriber: Pubkey,        // points to the subscriber state account
    pub subscription_plan: Pubkey, // points to the subscription plan account
    pub is_active: bool,           // true if subscription is active
    pub is_cancelled: bool,        // true if subscription was cancelled after being active

    // 1 = insufficent funds
    // 2 = delegation revoked
    // 3 = delegated amount not enough
    pub cancellation_reason: i8,

    pub last_payment_timestamp: i64, // last payment timstamp
    pub next_payment_timestamp: i64, // next payment timestamp
}
impl Subscription {
    pub fn space() -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // subscriber
        PUBKEY_SIZE + // subscription_plan
        1 + // is_active
        1 + // is_cancelled
        1 + // cancellation_reason
        8 + // last_payment_timestamp
        8  // next_payment_timestamp
    }
}


#[account]
pub struct SubscriptionPlan {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub plan_name: String,                         // subscription plan name
    pub subscription_plan_author: Pubkey,          // who creates the subscription plan
    pub subscription_plan_payment_account: Pubkey, // usdc wallet to recieve subscription payments
    pub amount: i64,                               // subscription amount
    pub frequency: i64,                            // subscription frequency
    pub is_active: bool,
    pub fee_percentage: i8,

    // This contains the PubKeys of all the subscription accounts that
    // interacted (subscribed/unsubscribed) with the subscription plan
    pub subscription_accounts: Vec<Pubkey>,
}
impl SubscriptionPlan {
    pub fn space(plan_name: &str) -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        4 + plan_name.len() + // plan_name
        PUBKEY_SIZE + // subscription_plan_author
        PUBKEY_SIZE + // subscription_plan_payment_account
        8 + // amount
        8 + // frequency
        1 + // is_active
        1 + // fee_percentage
        4 + (PUBKEY_SIZE * MAXIMUM_SUBSCRIPTIONS_PER_PLAN) // subscription_accounts
    }
}


#[account]
pub struct SubscriptionPlanAuthor {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey, // points to the account who creates the subscription

    // This contains the PubKeys of all the subsciption plans that the author has
    // interacted (created/closed) with
    pub subscription_plan_accounts: Vec<Pubkey>,
}
impl SubscriptionPlanAuthor {
    pub fn space() -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // authority
        4 + (PUBKEY_SIZE * MAXIMUM_SUBSCRIPTION_PLAN_PER_AUTHOR)  // subscription_plan_accounts
    }
}


#[account]
pub struct Protocol {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey,

    // This contains PubKeys of all the subscription plan accounts
    pub subscription_plan_accounts: Vec<Pubkey>,

    // This contains PubKeys of all the registered nodes
    pub registered_nodes: Vec<Pubkey>,
}
impl Protocol {
    pub fn space() -> usize {
        8 + // discriminator
        1 + // bump
        1 + // has_already_been_initialized
        PUBKEY_SIZE + // authority
        4 + (PUBKEY_SIZE * MAXIMUM_SUBSCRIPTION_PLANS) + // subscription_plan_accounts
        4 + (PUBKEY_SIZE * MAXIMUM_NODES)  // registered_nodes
    }
}

#[account]
pub struct ProtocolSigner {
    pub bump: u8,
}
impl ProtocolSigner {
    pub fn space() -> usize {
        8 + // discriminator
        1 // bump
    }
}


#[account]
pub struct Node {
    pub bump: u8,
    pub is_registered: bool,
    pub authority: Pubkey,
    pub node_payment_wallet: Pubkey,
    pub node_payment_account: Pubkey,
}
impl Node {
    pub fn space() -> usize {
        8 + // discriminator
        1 + // bump
        1 + // is_registered
        PUBKEY_SIZE + // authority
        PUBKEY_SIZE + // node_payment_wallet
        PUBKEY_SIZE // node_payment_account
    }
}