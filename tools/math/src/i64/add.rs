//! # `xyz.taluslabs.math.i64.add@1`
//!
//! Standard Nexus Tool that adds two [`i64`] numbers and returns the result.
//!
//! ## Input
//!
//! - `a`: [`i64`] - The first number to add.
//! - `b`: [`i64`] - The second number to add.
//!
//! ## Output Variants
//!
//! - `ok` - The addition was successful.
//! - `err` - The addition failed due to overflow.
//!
//! ## Output Ports
//!
//! ### `ok`
//!
//! - `result`: [`i64`] - The result of the addition.
//!
//! ### `err`
//!
//! - `reason`: [`String`] - The reason for the error. This is always overflow.

use {
    nexus_toolkit::*,
    nexus_types::*,
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Input {
    a: i64,
    b: i64,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Output {
    Ok { result: i64 },
    Err { reason: String },
}

pub(crate) struct I64Add;

impl NexusTool for I64Add {
    type Input = Input;
    type Output = Output;

    async fn new() -> Self {
        Self
    }

    fn fqn() -> ToolFqn {
        fqn!("xyz.taluslabs.math.i64.add@1")
    }

    fn path() -> &'static str {
        "/i64/add"
    }

    async fn health(&self) -> AnyResult<StatusCode> {
        // This tool has no external dependencies and as such, it is always
        // healthy if the endpoint is reachable.
        Ok(StatusCode::OK)
    }

    async fn invoke(&self, Self::Input { a, b }: Self::Input) -> Self::Output {
        match a.checked_add(b) {
            Some(result) => Output::Ok { result },
            None => Output::Err {
                reason: format!("Adding '{a}' and '{b}' results in an overflow"),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_i64_add() {
        let tool = I64Add::new().await;

        let input = Input { a: 1, b: 2 };
        let output = tool.invoke(input).await;

        assert!(matches!(output, Output::Ok { result: 3 }));

        let input = Input { a: i64::MAX, b: 1 };
        let output = tool.invoke(input).await;

        assert!(matches!(output, Output::Err { .. }));

        let input = Input { a: i64::MIN, b: -1 };
        let output = tool.invoke(input).await;

        assert!(matches!(output, Output::Err { .. }));
    }
}
