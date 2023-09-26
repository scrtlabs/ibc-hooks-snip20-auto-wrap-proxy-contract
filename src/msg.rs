use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    WrapDeposit {
        snip20_address: String,
        snip20_code_hash: String,
        recipient_address: String,
    },
}

/// SNIP20 token handle messages
#[derive(Serialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Snip20HandleMsg {
    Deposit {
        padding: Option<String>,
    },
    // Basic SNIP20 functions
    Transfer {
        recipient: String,
        amount: Uint128,
        memo: Option<String>,
        padding: Option<String>,
    },
}
