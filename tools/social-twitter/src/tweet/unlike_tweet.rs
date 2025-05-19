//! # `xyz.taluslabs.social.twitter.unlike-tweet@1`
//!
//! Standard Nexus Tool that unlikes a tweet.

use {
    super::models::UnlikeResponse,
    crate::{
        auth::TwitterAuth,
        error::TwitterErrorKind,
        twitter_client::{TwitterClient, TWITTER_API_BASE},
    },
    nexus_sdk::{fqn, ToolFqn},
    nexus_toolkit::*,
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
};

#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub(crate) struct Input {
    /// Twitter API credentials
    #[serde(flatten)]
    auth: TwitterAuth,
    /// The id of authenticated user
    user_id: String,
    /// Tweet ID to unlike
    tweet_id: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Output {
    Ok {
        /// Whether the tweet was unliked
        unliked: bool,
    },
    Err {
        /// Error message
        reason: String,
        /// Type of error (network, server, auth, etc.)
        kind: TwitterErrorKind,
        /// HTTP status code if available
        #[serde(skip_serializing_if = "Option::is_none")]
        status_code: Option<u16>,
    },
}

pub(crate) struct UnlikeTweet {
    api_base: String,
}

impl NexusTool for UnlikeTweet {
    type Input = Input;
    type Output = Output;

    async fn new() -> Self {
        Self {
            api_base: TWITTER_API_BASE.to_string(),
        }
    }

    fn fqn() -> ToolFqn {
        fqn!("xyz.taluslabs.social.twitter.unlike-tweet@1")
    }

    fn path() -> &'static str {
        "/unlike-tweet"
    }

    async fn health(&self) -> AnyResult<StatusCode> {
        Ok(StatusCode::OK)
    }

    async fn invoke(&self, request: Self::Input) -> Self::Output {
        // Build the endpoint for the Twitter API
        let suffix: String = format!("users/{}/likes/{}", request.user_id, request.tweet_id);

        let client = match TwitterClient::new(Some(&suffix), Some(&self.api_base)) {
            Ok(client) => client,
            Err(e) => {
                return Output::Err {
                    reason: e.to_string(),
                    kind: TwitterErrorKind::Network,
                    status_code: None,
                }
            }
        };

        match client.delete::<UnlikeResponse>(&request.auth).await {
            Ok(data) => Output::Ok {
                unliked: !data.liked,
            },
            Err(e) => Output::Err {
                reason: e.reason,
                kind: e.kind,
                status_code: e.status_code,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        ::{mockito::Server, serde_json::json},
    };

    impl UnlikeTweet {
        fn with_api_base(api_base: &str) -> Self {
            Self {
                api_base: api_base.to_string(),
            }
        }
    }

    async fn create_server_and_tool() -> (mockito::ServerGuard, UnlikeTweet) {
        let server = Server::new_async().await;
        let tool = UnlikeTweet::with_api_base(&server.url());
        (server, tool)
    }

    fn create_test_input() -> Input {
        Input {
            auth: TwitterAuth::new(
                "test_consumer_key",
                "test_consumer_secret",
                "test_access_token",
                "test_access_token_secret",
            ),
            user_id: "12345".to_string(),
            tweet_id: "67890".to_string(),
        }
    }

    #[tokio::test]
    async fn test_successful_unlike() {
        // Create server and tool
        let (mut server, tool) = create_server_and_tool().await;

        // Set up mock response for successful unlike
        let mock = server
            .mock("DELETE", "/users/12345/likes/67890")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "data": {
                        "liked": false
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        // Test the unlike request
        let result = tool.invoke(create_test_input()).await;

        // Verify the response
        match result {
            Output::Ok { unliked } => {
                assert_eq!(unliked, true);
            }
            Output::Err {
                reason,
                kind,
                status_code,
            } => panic!(
                "Expected success, got error: {} ({:?}, {:?})",
                reason, kind, status_code
            ),
        }

        // Verify that the mock was called
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_unauthorized_error() {
        // Create server and tool
        let (mut server, tool) = create_server_and_tool().await;

        // Set up mock for 401 Unauthorized response
        let mock = server
            .mock("DELETE", "/users/12345/likes/67890")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "detail": "Unauthorized",
                    "status": 401,
                    "title": "Unauthorized",
                    "type": "about:blank"
                })
                .to_string(),
            )
            .create_async()
            .await;

        // Test the unlike request
        let result = tool.invoke(create_test_input()).await;

        // Verify the error response
        match result {
            Output::Ok { .. } => panic!("Expected error, got success"),
            Output::Err {
                reason,
                kind,
                status_code,
            } => {
                // Check error type
                assert_eq!(
                    kind,
                    TwitterErrorKind::Auth,
                    "Expected error kind Auth, got: {:?}",
                    kind
                );

                // Check error message
                assert!(
                    reason.contains("Unauthorized"),
                    "Expected error message to contain 'Unauthorized', got: {}",
                    reason
                );

                // Check status code
                assert_eq!(
                    status_code,
                    Some(401),
                    "Expected status code 401, got: {:?}",
                    status_code
                );
            }
        }
        // Verify that the mock was called
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_invalid_json_response() {
        // Create server and tool
        let (mut server, tool) = create_server_and_tool().await;

        // Set up mock for invalid JSON response
        let mock = server
            .mock("DELETE", "/users/12345/likes/67890")
            .with_status(200)
            .with_body("invalid json")
            .create_async()
            .await;

        // Test the unlike request
        let result = tool.invoke(create_test_input()).await;

        // Verify the error response
        match result {
            Output::Ok { .. } => panic!("Expected error, got success"),
            Output::Err {
                reason,
                kind,
                status_code,
            } => {
                // Assertions for invalid JSON response (200 OK status)
                assert_eq!(
                    kind,
                    TwitterErrorKind::Parse, // Expect Parse error
                    "Expected error kind Parse, got: {:?}",
                    kind
                );
                assert!(
                    reason.contains("Response parsing error"), // Expect parsing error message
                    "Expected error message to contain 'Response parsing error', got: {}",
                    reason
                );
                assert_eq!(
                    status_code,
                    None, // Expect None status code for parsing errors
                    "Expected None status code for Parse error, got: {:?}",
                    status_code
                );
            }
        }

        // Verify that the mock was called
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_unexpected_format() {
        // Create server and tool
        let (mut server, tool) = create_server_and_tool().await;

        // Set up mock for unexpected response format
        let mock = server
            .mock("DELETE", "/users/12345/likes/67890")
            .with_status(200)
            .with_body(
                json!({
                    "data": {
                        "some_other_field": true
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        // Test the unlike request
        let result = tool.invoke(create_test_input()).await;

        // Verify the error response
        match result {
            Output::Ok { .. } => panic!("Expected error, got success"),
            Output::Err {
                reason,
                kind,
                status_code,
            } => {
                assert!(
                    reason.contains("Response parsing error"),
                    "Error should indicate response parsing error. Got: {}",
                    reason
                );
                assert_eq!(
                    kind,
                    TwitterErrorKind::Parse,
                    "Expected error kind Parse, got: {:?}",
                    kind
                );
                assert_eq!(status_code, None);
            }
        }

        // Verify that the mock was called
        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_tweet_already_unliked() {
        // Create server and tool
        let (mut server, tool) = create_server_and_tool().await;

        // Set up mock for tweet already liked response (API returns liked: true after unlike)
        let mock = server
            .mock("DELETE", "/users/12345/likes/67890")
            .with_status(200)
            .with_body(
                json!({
                    "data": {
                        "liked": true
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        // Test the unlike request
        let result = tool.invoke(create_test_input()).await;

        // Verify the response - Expect Ok with unliked: true
        match result {
            Output::Ok { unliked } => {
                assert_eq!(unliked, false); // Verify the API response is correctly passed through
            }
            Output::Err { reason, .. } => {
                panic!(
                    "Expected success (Ok with unliked: true), got error: {}",
                    reason
                );
            }
        }

        // Verify that the mock was called
        mock.assert_async().await;
    }
}
