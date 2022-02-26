use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

use instructions::*;

declare_id!("9Byr46LJDyAgDXzvAUmkpDzxjiwjC2rBuUn1m2AGC4oh");

#[program]
pub mod subrina_protocol {
    use super::*;

    pub fn initialize(ctx: Context<InitializeProtocol>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx)
    }

    pub fn initialize_subscriber(ctx: Context<InitializeSubscriberAccount>) -> Result<()> {
        instructions::initialize_subscriber::handler(ctx)
    }

    pub fn subscribe(ctx: Context<Subscribe>, how_many_cycles: i64) -> Result<()> {
        instructions::subscribe::handler(ctx, how_many_cycles)
    }

    pub fn unsubscribe(ctx: Context<Unsubscribe>) -> Result<()> {
        instructions::unsubscribe::handler(ctx)
    }

    pub fn create_subscription_plan(
        ctx: Context<CreateSubscriptionPlan>,
        plan_name: String,
        subscription_amount: i64,
        frequency: i64,
    ) -> Result<()> {
        instructions::create_subscription_plan::handler(
            ctx,
            plan_name,
            subscription_amount,
            frequency,
        )
    }

    pub fn close_subscription_plan(ctx: Context<CloseSubscriptionPlan>) -> Result<()> {
        instructions::close_subscription_plan::handler(ctx)
    }

    pub fn try_take_payment(ctx: Context<TakePayment>) -> Result<()> {
        instructions::take_payment::handler(ctx)
    }
}
