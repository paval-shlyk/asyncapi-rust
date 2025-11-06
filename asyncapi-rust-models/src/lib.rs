//! Runtime data structures for AsyncAPI 3.0 specifications
//!
//! This crate provides Rust types that represent AsyncAPI 3.0 specification objects.
//! These types are used by the proc macros to generate specifications at compile time.

#![deny(missing_docs)]
#![warn(clippy::all)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// AsyncAPI 3.0 Specification
///
/// Root document object representing a complete AsyncAPI specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsyncApiSpec {
    /// AsyncAPI version (e.g., "3.0.0")
    pub asyncapi: String,

    /// General information about the API
    pub info: Info,

    /// Server connection details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<HashMap<String, Server>>,

    /// Available channels (communication paths)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<HashMap<String, Channel>>,

    /// Operations (send/receive)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<HashMap<String, Operation>>,

    /// Reusable components (messages, schemas, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub components: Option<Components>,
}

/// API information object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    /// API title
    pub title: String,

    /// API version
    pub version: String,

    /// API description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Server connection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Server {
    /// Server URL or host
    pub host: String,

    /// Protocol (e.g., "wss", "ws", "grpc")
    pub protocol: String,

    /// Server description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Communication channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    /// Channel address/path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// Messages available on this channel
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<HashMap<String, MessageRef>>,
}

/// Reference to a message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MessageRef {
    /// Reference to component message
    Reference {
        /// $ref path
        #[serde(rename = "$ref")]
        reference: String,
    },
    /// Inline message definition
    Inline(Box<Message>),
}

/// Message definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Message title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Message summary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Message description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Content type (e.g., "application/json")
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,

    /// Message payload schema
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Schema>,
}

/// Operation (send or receive)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// Operation action (send or receive)
    pub action: OperationAction,

    /// Channel reference
    pub channel: ChannelRef,

    /// Messages for this operation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<MessageRef>>,
}

/// Operation action type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OperationAction {
    /// Send message
    Send,
    /// Receive message
    Receive,
}

/// Reference to a channel
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelRef {
    /// $ref path
    #[serde(rename = "$ref")]
    pub reference: String,
}

/// Reusable components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Components {
    /// Message definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<HashMap<String, Message>>,

    /// Schema definitions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<HashMap<String, Schema>>,
}

/// JSON Schema object
///
/// Simplified representation - will be expanded as needed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schema {
    /// Schema type
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,

    /// Properties (for object type)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Schema>>,

    /// Required properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,

    /// Description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl Default for AsyncApiSpec {
    fn default() -> Self {
        Self {
            asyncapi: "3.0.0".to_string(),
            info: Info {
                title: "API".to_string(),
                version: "1.0.0".to_string(),
                description: None,
            },
            servers: None,
            channels: None,
            operations: None,
            components: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spec_serialization() {
        let spec = AsyncApiSpec::default();
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("asyncapi"));
        assert!(json.contains("3.0.0"));
    }

    #[test]
    fn test_spec_deserialization() {
        let json = r#"{
            "asyncapi": "3.0.0",
            "info": {
                "title": "Test API",
                "version": "1.0.0"
            }
        }"#;
        let spec: AsyncApiSpec = serde_json::from_str(json).unwrap();
        assert_eq!(spec.asyncapi, "3.0.0");
        assert_eq!(spec.info.title, "Test API");
    }
}
