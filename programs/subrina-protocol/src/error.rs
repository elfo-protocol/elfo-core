use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    // subscriber errors ----
    #[msg("Subscriber is not initialized.")]
    SubscriberNotInitialized,

    // subscription errors ----
    #[msg("Subscription is not initialized.")]
    SubscriptionNotInitialized,

    #[msg("User is already subscribed to the plan.")]
    SubscriptionAlreadySubscribed,

    #[msg("User is not subscribed to the plan.")]
    SubscriptionNotSubscribed,

    #[msg("Not enough funds in protocol wallet to subscribe.")]
    SubscriptionNotEnoughFunds,

    #[msg("Next payment timestamp not reached. Please try again later.")]
    SubscriptionNextPaymentTimestampNotReached,

    // subscription plan errors -----
    #[msg("Subscription plan is not initialized.")]
    SubscriptionPlanNotInitialized,

    #[msg("Subscription amount must be in the range of 1 - 1000 USDC.")]
    SubscriptionPlanAmountInvalid,

    #[msg("Subscription plan is inactive.")]
    SubscriptionPlanInactive,

    #[msg("Subscription plan is already closed.")]
    SubscriptionPlanAlreadyClosed,

    // note: 60 second may not be ideal
    #[msg("Subscription plan frequency must be atleast 60 seconds.")]
    SubscriptionPlanFrequencyError,

    #[msg("Unauthorized to close subscription.")]
    SubscriptionPlanUnauthorizedToClose,

    #[msg("Invalid payment account provided.")]
    SubscriptionPlanInvalidPaymentAccount,

    // token error ----
    #[msg("Invalid mint.")]
    InvalidMint,
}
