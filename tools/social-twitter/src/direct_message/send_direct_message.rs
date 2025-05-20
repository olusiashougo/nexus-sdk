//! # `xyz.taluslabs.social.twitter.send-direct-message@1`
//!
//! Standard Nexus Tool that sends a direct message to a user.

use {
    super::models::{Attachment, DirectMessageResponse},
    crate::{
        auth::TwitterAuth,
        error::TwitterErrorKind,
        twitter_client::{TwitterClient, TWITTER_API_BASE},
    },
    nexus_sdk::{fqn, ToolFqn},
    nexus_toolkit::*,
    schemars::JsonSchema,
    serde::{Deserialize, Serialize},
    serde_json::json,
};

#[derive(Deserialize, JsonSchema)]
pub(crate) struct Input {
    /// The access token received from the authorization server in the OAuth 2.0 flow.
    #[serde(flatten)]
    auth: TwitterAuth,
    /// The ID of the recipient user that will receive the DM.
    participant_id: String,
    /// Text of the message.
    text: Option<String>,
    /// Attachments to a DM Event.
    attachments: Option<Vec<Attachment>>,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Output {
    Ok {
        /// Unique identifier of a DM conversation.
        /// This can either be a numeric string, or a pair of numeric strings separated by a '-' character in the case of one-on-one DM Conversations.
        dm_conversation_id: String,
        /// Unique identifier of a DM Event.
        dm_event_id: String,
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

pub(crate) struct SendDirectMessage {
    api_base: String,
}

impl NexusTool for SendDirectMessage {
    type Input = Input;
    type Output = Output;

    async fn new() -> Self {
        Self {
            api_base: TWITTER_API_BASE.to_string(),
        }
    }

    // /:participant_id/dm_events
    fn fqn() -> ToolFqn {
        fqn!("xyz.taluslabs.social.twitter.send-direct-message@1")
    }

    fn path() -> &'static str {
        "/send-direct-message"
    }

    async fn health(&self) -> AnyResult<StatusCode> {
        Ok(StatusCode::OK)
    }

    async fn invoke(&self, request: Self::Input) -> Self::Output {
        // Build the endpoint for the Twitter API
        let suffix = format!("dm_conversations/with/{}/messages", request.participant_id);

        // Create a Twitter client with the endpoint
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

        // Construct request body based on input
        let mut request_body = json!({});

        if let Some(text) = &request.text {
            request_body["text"] = json!(text);
        }

        if let Some(attachments) = &request.attachments {
            let media_attachments: Vec<serde_json::Value> = attachments
                .iter()
                .map(|a| json!({ "media_id": a.media_id }))
                .collect();
            request_body["attachments"] = json!(media_attachments);
        }

        // Make the request to Twitter API
        match client
            .post::<DirectMessageResponse, _>(&request.auth, request_body)
            .await
        {
            Ok(data) => Output::Ok {
                dm_conversation_id: data.dm_conversation_id,
                dm_event_id: data.dm_event_id,
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
    use {super::*, ::mockito::Server, serde_json::json};

    impl SendDirectMessage {
        fn with_api_base(api_base: &str) -> Self {
            Self {
                api_base: api_base.to_string(),
            }
        }
    }

    async fn create_server_and_tool() -> (mockito::ServerGuard, SendDirectMessage) {
        let server = Server::new_async().await;
        let tool = SendDirectMessage::with_api_base(&server.url());
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
            participant_id: "12345".to_string(),
            text: Some("Test message".to_string()),
            attachments: None,
        }
    }

    #[tokio::test]
    async fn test_send_direct_message_successful() {
        let (mut server, tool) = create_server_and_tool().await;

        let mock = server
            .mock("POST", "/dm_conversations/with/12345/messages")
            .match_header("content-type", "application/json")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "data": {
                        "dm_conversation_id": "123123123-456456456",
                        "dm_event_id": "1146654567674912769"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let output = tool.invoke(create_test_input()).await;

        match output {
            Output::Ok {
                dm_conversation_id,
                dm_event_id,
            } => {
                assert_eq!(dm_conversation_id, "123123123-456456456");
                assert_eq!(dm_event_id, "1146654567674912769");
            }
            Output::Err { reason, kind, .. } => {
                panic!("Expected success, got error: {:?} - {}", kind, reason)
            }
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_direct_message_with_attachments() {
        let (mut server, tool) = create_server_and_tool().await;

        let input = Input {
            auth: TwitterAuth::new(
                "test_consumer_key",
                "test_consumer_secret",
                "test_access_token",
                "test_access_token_secret",
            ),
            participant_id: "12345".to_string(),
            text: Some("Test message with attachment".to_string()),
            attachments: Some(vec![
                Attachment {
                    media_id: "1146654567674912769".to_string(),
                },
                Attachment {
                    media_id: "1146654567674912770".to_string(),
                },
                Attachment {
                    media_id: "1146654567674912771".to_string(),
                },
            ]),
        };

        let mock = server
            .mock("POST", "/dm_conversations/with/12345/messages")
            .match_header("content-type", "application/json")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "data": {
                        "dm_conversation_id": "123123123-456456456",
                        "dm_event_id": "1146654567674912769"
                    }
                })
                .to_string(),
            )
            .create_async()
            .await;

        let output = tool.invoke(input).await;

        match output {
            Output::Ok {
                dm_conversation_id,
                dm_event_id,
            } => {
                assert_eq!(dm_conversation_id, "123123123-456456456");
                assert_eq!(dm_event_id, "1146654567674912769");
            }
            Output::Err { reason, kind, .. } => {
                panic!("Expected success, got error: {:?} - {}", kind, reason)
            }
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_direct_message_unauthorized() {
        let (mut server, tool) = create_server_and_tool().await;

        let mock = server
            .mock("POST", "/dm_conversations/with/12345/messages")
            .match_header("content-type", "application/json")
            .with_status(401)
            .with_header("content-type", "application/json")
            .with_body(
                json!({
                    "title": "Unauthorized",
                    "type": "https://api.twitter.com/2/problems/unauthorized",
                    "status": 401,
                    "detail": "Unauthorized"
                })
                .to_string(),
            )
            .create_async()
            .await;

        let output = tool.invoke(create_test_input()).await;

        match output {
            Output::Ok { .. } => panic!("Expected error, got success"),
            Output::Err {
                reason,
                kind,
                status_code,
            } => {
                assert_eq!(kind, TwitterErrorKind::Auth);
                assert!(
                    reason.contains("Unauthorized"),
                    "Expected error message to contain 'Unauthorized', got: {}",
                    reason
                );
                assert_eq!(status_code, Some(401));
            }
        }

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_send_direct_message_invalid_json() {
        let (mut server, tool) = create_server_and_tool().await;

        let mock = server
            .mock("POST", "/dm_conversations/with/12345/messages")
            .match_header("content-type", "application/json")
            .with_status(200)
            .with_body("invalid json")
            .create_async()
            .await;

        let output = tool.invoke(create_test_input()).await;

        match output {
            Output::Ok { .. } => panic!("Expected error, got success"),
            Output::Err {
                reason,
                kind,
                status_code: _,
            } => {
                assert_eq!(kind, TwitterErrorKind::Parse);
                assert!(
                    reason.contains("Response parsing error"),
                    "Error should indicate parsing error. Got: {}",
                    reason
                );
            }
        }

        mock.assert_async().await;
    }
}
