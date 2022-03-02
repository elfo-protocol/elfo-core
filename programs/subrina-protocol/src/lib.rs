use anchor_lang::prelude::*;

pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("FWrg3R4FVkLDaxcA6uYsGhV4hDpKWxu7AgoFUuWGKYUP");

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
        fee_percentage: i8,
    ) -> Result<()> {
        instructions::create_subscription_plan::handler(
            ctx,
            plan_name,
            subscription_amount,
            frequency,
            fee_percentage,
        )
    }

    pub fn close_subscription_plan(ctx: Context<CloseSubscriptionPlan>) -> Result<()> {
        instructions::close_subscription_plan::handler(ctx)
    }

    pub fn try_take_payment(ctx: Context<TakePayment>) -> Result<()> {
        instructions::take_payment::handler(ctx)
    }

    pub fn register_node(ctx: Context<RegisterNode>) -> Result<()> {
        instructions::register_node::handler(ctx)
    }
}
