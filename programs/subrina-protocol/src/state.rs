use anchor_lang::prelude::*;

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

#[account]
pub struct SubscriptionPlanAuthor {
    pub bump: u8,
    pub has_already_been_initialized: bool,
    pub authority: Pubkey, // points to the account who creates the subscription

    // This contains the PubKeys of all the subsciption plans that the author has
    // interacted (created/closed) with
    pub subscription_plan_accounts: Vec<Pubkey>,
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

#[account]
pub struct ProtocolSigner {
    pub bump: u8,
}

#[account]
pub struct Node {
    pub is_registered: bool,
    pub bump: u8,
    pub authority: Pubkey,
    pub node_payment_wallet: Pubkey,
    pub node_payment_account: Pubkey,
}
