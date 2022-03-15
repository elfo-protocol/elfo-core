<div align="center">
  <img height="170x" src="https://i.imgur.com/DfZCJAd.png?width=746&height=746" />

  <h1>Elfo Protocol</h1>

  <p>
    <strong>Protocol for Subscription Payments on Solana</strong>
  </p>

  <p>
    <a href="https://github.com/elfo-protocol/elfo-sdk"><img alt="Tutorials" src="https://img.shields.io/badge/sdk-javascript-informational" /></a>
    <a href="https://discord.gg/QT3WgFrC"><img alt="Discord Chat" src="https://img.shields.io/discord/951843923649769522?color=yellowgreen" /></a>
    <a href="https://opensource.org/licenses/Apache-2.0"><img alt="License" src="https://img.shields.io/github/license/elfo-protocol/elfo-core?color=blueviolet" /></a>
  </p>
</div>

Elfo Protocol enables subscription payment on Solana blockchain. 

The protocol consists of three main componenets.

### Core program (smart contract)
[This Repository]


### Elfo node CLI
[ [github.com/elfo-protocol/elfo-node](https://github.com/elfo-protocol/elfo-node) ]

### Elfo Javascript SDK
[[github.com/elfo-protocol/elfo-sdk](https://github.com/elfo-protocol/elfo-sdk)]

---

# Core Program

This repository contains the core program of the elfo protocol.

To use `elfo-ptotocol` for CPI, add `elfo-protocol-core` under `[dependencies]` in `cargo.toml` file. Make sure `"cpi"` feature is enabled.


```
[dependencies]
...
elfo-protocol-core = {version="0.1.0", features=["cpi"]}
```


Instructions listed bellow makes up the protocol.

### create_subscription_plan

Creates a subscription plan.

**create_subscription_plan**(
    `plan_name`, 
    `subscription_amount`,
    `frequency`,
    `feePercentage`
):

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `plan_name` | `String` | A string mentioning a name for subscription plan |
| `subscription_amount` | `i64` |  Subscription amount in USDC with decimals |
| `frequency` | `i64` | Subscription frequency in seconds |
| `feePercentage` | `i8` | An integer between 1 - 5 that specifies the percentage of the fees that goes to nodes. The higher the percentage, the more incentive for nodes to monitor and trigger payments. |

#### Accounts

| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `authority` |The account which creates the subscription plan. | ***Signer***|
| `protocol_state` | The elfo protocol state account. |[ELFO_PROTOCOL_STATE](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/modules.md#elfo_protocol_state) |
| `subscription_plan_author` |  Subscription payment author account (*init-if-needed*). | [SubscriptionPlanAuthor.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlanAuthor.md#address) |
| `subscription_plan` |  Subscription plan account (*init*). | [SubscriptionPlan.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlan.md#address) |
| `subscription_plan_payment_account` | USDC Associated Token account of subscription author to recieve payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `mint` | USDC mint account. | [USDC Mint](EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v)|
| `token_program` | Token Program | [TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#TOKEN_PROGRAM_ID)
| `associated_token_program` | Associated Token Program | [ASSOCIATED_TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#ASSOCIATED_TOKEN_PROGRAM_ID)
| `system_program` | System Program | [SystemProgram.programId](https://solana-labs.github.io/solana-web3.js/classes/SystemProgram.html#programId)
| `rent`| Rent Program | [SYSVAR_RENT_PUBKEY](https://solana-labs.github.io/solana-web3.js/modules.html#SYSVAR_RENT_PUBKEY)

### subscribe

Delegates (approve) required tokens  and subscribes to subscription plan. 

***subscribe*** ( `how_many_cycles` )

#### Parameters

| Name | Type | Description |
| :------ | :------ | :------ |
| `how_many_cycles` | `i64` | How many cycles should the funds be delegated to |

#### Accounts

| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `who_subscribes` | The account which subscribes to the plan. | ***Signer***|
| `protocol_signer` | The elfo protocol signer account. |[ELFO_PROTOCOL_SIGNER](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/modules.md#elfo_protocol_signer) |
| `subscription` |  Subscription account (init_if_needed) | [Subscription.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/Subscription.md#address) |
| `subscriber` |  Subscriber account (init_if_needed) | [Subscriber.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/Subscriber.md#address) |
| `subscriber_payment_account` | USDC Associated Token account of subscriber to delegate payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `subscription_plan` |  Subscription plan account | [SubscriptionPlan.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlan.md#address) |
| `subscription_plan_payment_account` | USDC Associated Token account of subscription author to recieve payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `mint` | USDC mint account. | [USDC Mint](EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v)|
| `token_program` | Token Program | [TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#TOKEN_PROGRAM_ID)
| `associated_token_program` | Associated Token Program | [ASSOCIATED_TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#ASSOCIATED_TOKEN_PROGRAM_ID)
| `system_program` | System Program | [SystemProgram.programId](https://solana-labs.github.io/solana-web3.js/classes/SystemProgram.html#programId)
| `rent`| Rent Program | [SYSVAR_RENT_PUBKEY](https://solana-labs.github.io/solana-web3.js/modules.html#SYSVAR_RENT_PUBKEY)
| `clock`| Clock Program | [SYSVAR_CLOCK_PUBKEY](https://solana-labs.github.io/solana-web3.js/modules.html#SYSVAR_CLOCK_PUBKEY)

### register_node

Registers a Elfo node to the protocol. Only registered node can monitor and trigger payment to earn fees.

***register_node*** ()

#### Parameters

None

#### Accounts

| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `authority` |The account which registers the node (node authority) | ***Signer***|
| `node` | Node account (init_if_needed) |[ElfoNode.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/ElfoNode.md#address) |
| `node_payment_account` | USDC Associated Token account of `node_payment_wallet` to recieve USDC fee payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |[getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `protocol_state` | The elfo protocol state account. |[ELFO_PROTOCOL_STATE](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/modules.md#elfo_protocol_state) |
| `node_payment_wallet` | Wallet account used to get and verify `node_payment_account` | |
| `mint` | USDC mint account. | [USDC Mint](EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v)|
| `token_program` | Token Program | [TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#TOKEN_PROGRAM_ID)
| `associated_token_program` | Associated Token Program | [ASSOCIATED_TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#ASSOCIATED_TOKEN_PROGRAM_ID)
| `system_program` | System Program | [SystemProgram.programId](https://solana-labs.github.io/solana-web3.js/classes/SystemProgram.html#programId)
| `rent`| Rent Program | [SYSVAR_RENT_PUBKEY](https://solana-labs.github.io/solana-web3.js/modules.html#SYSVAR_RENT_PUBKEY)

### trigger_payment
Trigger a payment on a subscription. This is called by registered elfo-nodes.

***trigger_payment*** ()

#### Parameters

None

#### Accounts
| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `authority` |The account which registered the node (node authority) | ***Signer***|
| `subscriber_payment_account` | USDC Associated Token account of subscriber to delegate payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `protocol_signer` | The elfo protocol signer account. |[ELFO_PROTOCOL_SIGNER](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/modules.md#elfo_protocol_signer) |
| `subscription` |  Subscription account | [Subscription.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/Subscription.md#address) |
| `subscriber` |  Subscriber account| [Subscriber.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/Subscriber.md#address) |
| `subscription_plan_payment_account` | USDC Associated Token account of subscription author to recieve payments. | [getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `subscription_plan` |  Subscription plan account. | [SubscriptionPlan.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlan.md#address) |
| `node` | Node account |[ElfoNode.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/ElfoNode.md#address) |
| `node_payment_account` | USDC Associated Token account of `node_payment_wallet` to recieve USDC fee payments. |[getAssociateTokenAddress](https://solana-labs.github.io/solana-program-library/token/js/modules.html#getAssociatedTokenAddress) |
| `node_payment_wallet` | Wallet account used to get and verify `node_payment_account` | |
| `mint` | USDC mint account. | [USDC Mint](EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v)|
| `token_program` | Token Program | [TOKEN_PROGRAM_ID](https://solana-labs.github.io/solana-program-library/token/js/modules.html#TOKEN_PROGRAM_ID)
| `clock`| Clock Program | [SYSVAR_CLOCK_PUBKEY](https://solana-labs.github.io/solana-web3.js/modules.html#SYSVAR_CLOCK_PUBKEY)

### unsubscribe
Unsubscribe from a subscription plan

***unsubscribe*** ()

#### Parameters

None

#### Accounts

| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `who_unsubscribes` | The account which unsubscribes from the plan. | ***Signer***|
| `subscriber` |  Subscriber account | [Subscriber.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/Subscriber.md#address) |
| `subscription_plan` |  Subscription plan account. | [SubscriptionPlan.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlan.md#address) |

### close_subscription_plan
Closes a subscription plan

***close_subscription_plan*** ()

#### Parameters

None

#### Accounts
| Name | Description | References & Notes|
| :------ | :------ | :------ |
| `authority` |The account which initially created the subscription plan | ***Signer***|
| `subscription_plan_author` |  Subscription payment author account. | [SubscriptionPlanAuthor.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlanAuthor.md#address) |
| `subscription_plan` |  Subscription plan account. | [SubscriptionPlan.address](https://github.com/elfo-protocol/elfo-sdk/blob/master/docs/classes/SubscriptionPlan.md#address) |

---
## Note

* **Elfo Protocol is in active development, so all APIs are subject to change.**
* **Elfo protocol only supports USDC-SPL payments right now. In future this will be extended to support any SPL tokens**
* **This code is not audited yet. Use at your own risk.**

## License

Elfo Protocol is licensed under [Apache 2.0](./LICENSE).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Anchor by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.