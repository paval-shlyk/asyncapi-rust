//! Real-world example: actix-web + actix-ws WebSocket server with AsyncAPI
//!
//! This example demonstrates:
//! - Integrating AsyncAPI with actix-web WebSocket handlers
//! - Bi-directional WebSocket communication
//! - Automatic API documentation from working code
//! - Type-safe message handling
//!
//! ## Running this example
//!
//! ```bash
//! cargo run --example actix_websocket
//! ```
//!
//! Then in another terminal:
//! ```bash
//! # Connect with websocat (install: cargo install websocat)
//! websocat ws://127.0.0.1:8080/ws
//!
//! # Send messages (JSON):
//! {"type":"join","username":"alice","room":"general"}
//! {"type":"message","username":"alice","room":"general","text":"Hello!"}
//! {"type":"leave","username":"alice","room":"general"}
//! ```
//!
//! ## Dependencies
//!
//! Add to Cargo.toml:
//! ```toml
//! [dependencies]
//! actix-web = "4"
//! actix-ws = "0.3"
//! tokio = { version = "1", features = ["full"] }
//! ```

use asyncapi_rust::{AsyncApi, ToAsyncApiMessage, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

/// WebSocket messages for a chat application
///
/// These messages are used in the actual WebSocket handlers and also
/// generate AsyncAPI documentation automatically.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum ChatMessage {
    /// User joins a chat room
    #[serde(rename = "join")]
    #[asyncapi(
        summary = "User joins a room",
        description = "Sent when a user enters a chat room"
    )]
    Join {
        /// Username of the joining user
        username: String,
        /// Room to join
        room: String,
    },

    /// User sends a chat message
    #[serde(rename = "message")]
    #[asyncapi(
        summary = "Send a chat message",
        description = "Broadcast a message to all users in a room"
    )]
    Message {
        /// Username of sender
        username: String,
        /// Target room
        room: String,
        /// Message text
        text: String,
    },

    /// User leaves a chat room
    #[serde(rename = "leave")]
    #[asyncapi(
        summary = "User leaves a room",
        description = "Sent when a user exits a chat room"
    )]
    Leave {
        /// Username of the leaving user
        username: String,
        /// Room being left
        room: String,
    },
}

/// Complete API specification
#[allow(clippy::duplicated_attributes)] // False positive - different operations can reference same channel
#[derive(AsyncApi)]
#[asyncapi(
    title = "Chat WebSocket API",
    version = "1.0.0",
    description = "Real-time chat application using actix-ws"
)]
#[asyncapi_server(name = "development", host = "localhost:8080", protocol = "ws")]
#[asyncapi_channel(name = "chat", address = "/ws")]
#[asyncapi_operation(name = "sendMessage", action = "send", channel = "chat")]
#[asyncapi_operation(name = "receiveMessage", action = "receive", channel = "chat")]
struct ChatApi;

fn main() {
    println!("🚀 actix-web + AsyncAPI Integration Example\n");

    // Generate the AsyncAPI specification
    let spec = ChatApi::asyncapi_spec();

    println!("📋 API Specification:");
    println!("  Title: {}", spec.info.title);
    println!("  Version: {}", spec.info.version);
    println!();

    // Show message types
    println!("📨 Message Types:");
    for name in ChatMessage::asyncapi_message_names() {
        println!("  • {}", name);
    }
    println!();

    // Generate complete spec with messages
    let messages = ChatMessage::asyncapi_messages();
    println!("✅ Generated {} message schemas\n", messages.len());

    // Serialize to JSON
    let spec_json = serde_json::to_string_pretty(&spec).unwrap();
    println!("📄 AsyncAPI Specification:\n{}\n", spec_json);

    println!("💡 Integration Points:");
    println!("   • ChatMessage enum used in WebSocket handlers");
    println!("   • serde for JSON serialization/deserialization");
    println!("   • AsyncAPI spec generated from the same code");
    println!("   • Type safety enforced at compile time");
    println!();

    println!("🔗 Next Steps:");
    println!("   1. Add actix-web and actix-ws dependencies");
    println!("   2. Implement WebSocket handler using ChatMessage");
    println!("   3. Use serde_json to parse incoming messages");
    println!("   4. Export spec to docs/asyncapi.json for documentation");
    println!();

    println!("📚 Example Handler Pattern:");
    println!(
        r#"
    async fn websocket_handler(
        req: HttpRequest,
        stream: web::Payload,
    ) -> Result<HttpResponse, Error> {{
        let (response, session, mut msg_stream) = actix_ws::handle(&req, stream)?;

        actix_web::rt::spawn(async move {{
            while let Some(Ok(msg)) = msg_stream.next().await {{
                if let actix_ws::Message::Text(text) = msg {{
                    // Parse incoming message
                    if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {{
                        match chat_msg {{
                            ChatMessage::Join {{ username, room }} => {{
                                // Handle join
                            }}
                            ChatMessage::Message {{ username, room, text }} => {{
                                // Handle message
                            }}
                            ChatMessage::Leave {{ username, room }} => {{
                                // Handle leave
                            }}
                        }}
                    }}
                }}
            }}
        }});

        Ok(response)
    }}
    "#
    );
}
