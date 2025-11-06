//! Real-world example: axum WebSocket server with AsyncAPI
//!
//! This example demonstrates:
//! - Integrating AsyncAPI with axum WebSocket handlers
//! - Bi-directional WebSocket communication
//! - Automatic API documentation from working code
//! - Type-safe message handling with axum extractors
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example axum_websocket
//! ```
//!
//! Then in another terminal:
//! ```bash
//! # Connect with websocat (install: cargo install websocat)
//! websocat ws://127.0.0.1:3000/ws
//!
//! # Send messages (JSON):
//! {"type":"subscribe","topics":["news","sports"]}
//! {"type":"unsubscribe","topics":["sports"]}
//! {"type":"publish","topic":"news","data":"Breaking news!"}
//! ```
//!
//! ## Dependencies
//!
//! Add to Cargo.toml:
//! ```toml
//! [dependencies]
//! axum = { version = "0.7", features = ["ws"] }
//! tokio = { version = "1", features = ["full"] }
//! tower = "0.4"
//! ```

use asyncapi_rust::{AsyncApi, ToAsyncApiMessage, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

/// WebSocket messages for a pub/sub system
///
/// These messages define the protocol for a topic-based publish/subscribe
/// system over WebSocket. They're used in actual handlers and generate
/// AsyncAPI documentation.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum PubSubMessage {
    /// Subscribe to topics
    #[serde(rename = "subscribe")]
    #[asyncapi(
        summary = "Subscribe to topics",
        description = "Client subscribes to one or more topics to receive updates"
    )]
    Subscribe {
        /// List of topics to subscribe to
        topics: Vec<String>,
    },

    /// Unsubscribe from topics
    #[serde(rename = "unsubscribe")]
    #[asyncapi(
        summary = "Unsubscribe from topics",
        description = "Client unsubscribes from topics"
    )]
    Unsubscribe {
        /// List of topics to unsubscribe from
        topics: Vec<String>,
    },

    /// Publish a message to a topic
    #[serde(rename = "publish")]
    #[asyncapi(
        summary = "Publish to a topic",
        description = "Publish data to a specific topic"
    )]
    Publish {
        /// Target topic
        topic: String,
        /// Message payload
        data: String,
    },

    /// Notification of a published message
    #[serde(rename = "notification")]
    #[asyncapi(
        summary = "Topic notification",
        description = "Server sends notifications when subscribed topics receive messages"
    )]
    Notification {
        /// Topic that received the message
        topic: String,
        /// Message payload
        data: String,
    },

    /// Error message
    #[serde(rename = "error")]
    #[asyncapi(
        summary = "Error notification",
        description = "Server sends error messages when operations fail"
    )]
    Error {
        /// Error code
        code: String,
        /// Error description
        message: String,
    },
}

/// Complete API specification
#[allow(clippy::duplicated_attributes)] // False positive - different operations can reference same channel
#[derive(AsyncApi)]
#[asyncapi(
    title = "PubSub WebSocket API",
    version = "1.0.0",
    description = "Real-time publish/subscribe system using axum WebSocket"
)]
#[asyncapi_server(
    name = "development",
    host = "localhost:3000",
    protocol = "ws",
    description = "Development server"
)]
#[asyncapi_channel(name = "pubsub", address = "/ws")]
#[asyncapi_operation(name = "clientSend", action = "send", channel = "pubsub")]
#[asyncapi_operation(name = "serverSend", action = "receive", channel = "pubsub")]
struct PubSubApi;

fn main() {
    println!("🚀 axum WebSocket + AsyncAPI Integration Example\n");

    // Generate the AsyncAPI specification
    let spec = PubSubApi::asyncapi_spec();

    println!("📋 API Specification:");
    println!("  Title: {}", spec.info.title);
    println!("  Version: {}", spec.info.version);
    if let Some(desc) = &spec.info.description {
        println!("  Description: {}", desc);
    }
    println!();

    // Show server configuration
    if let Some(servers) = &spec.servers {
        println!("🖥️  Servers:");
        for (name, server) in servers {
            println!("  • {} - {}://{}", name, server.protocol, server.host);
        }
        println!();
    }

    // Show message types
    println!("📨 Message Types:");
    for name in PubSubMessage::asyncapi_message_names() {
        println!("  • {}", name);
    }
    println!();

    // Generate complete spec with messages
    let messages = PubSubMessage::asyncapi_messages();
    println!("✅ Generated {} message schemas\n", messages.len());

    // Serialize to JSON
    let spec_json = serde_json::to_string_pretty(&spec).unwrap();
    println!("📄 AsyncAPI Specification:\n{}\n", spec_json);

    println!("💡 Integration Points:");
    println!("   • PubSubMessage enum used in axum WebSocket handlers");
    println!("   • Type-safe extractors with axum::extract::ws");
    println!("   • AsyncAPI spec generated from the same code");
    println!("   • Compile-time validation of message types");
    println!();

    println!("🔗 Next Steps:");
    println!("   1. Add axum and tokio dependencies");
    println!("   2. Implement WebSocket handler using PubSubMessage");
    println!("   3. Use serde_json to parse/serialize messages");
    println!("   4. Export spec to docs/asyncapi.json");
    println!();

    println!("📚 Example Handler Pattern:");
    println!(
        r#"
    use axum::{{
        extract::ws::{{WebSocket, WebSocketUpgrade, Message}},
        response::Response,
        routing::get,
        Router,
    }};

    async fn websocket_handler(ws: WebSocketUpgrade) -> Response {{
        ws.on_upgrade(handle_socket)
    }}

    async fn handle_socket(mut socket: WebSocket) {{
        while let Some(Ok(msg)) = socket.recv().await {{
            if let Message::Text(text) = msg {{
                // Parse incoming message
                match serde_json::from_str::<PubSubMessage>(&text) {{
                    Ok(PubSubMessage::Subscribe {{ topics }}) => {{
                        // Handle subscribe
                    }}
                    Ok(PubSubMessage::Publish {{ topic, data }}) => {{
                        // Handle publish
                    }}
                    Ok(PubSubMessage::Unsubscribe {{ topics }}) => {{
                        // Handle unsubscribe
                    }}
                    _ => {{
                        // Send error response
                        let error = PubSubMessage::Error {{
                            code: "INVALID_MESSAGE".to_string(),
                            message: "Unknown message type".to_string(),
                        }};
                        let _ = socket.send(Message::Text(
                            serde_json::to_string(&error).unwrap()
                        )).await;
                    }}
                }}
            }}
        }}
    }}

    #[tokio::main]
    async fn main() {{
        let app = Router::new().route("/ws", get(websocket_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }}
    "#
    );
}
