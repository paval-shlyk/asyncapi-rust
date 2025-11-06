//! Framework Integration Guide
//!
//! This example provides a comprehensive guide for integrating asyncapi-rust
//! with popular Rust web frameworks. It shows practical patterns and best practices.

use asyncapi_rust::{AsyncApi, ToAsyncApiMessage, schemars::JsonSchema};
use serde::{Deserialize, Serialize};

/// Example: Type-safe WebSocket messages
///
/// Define your WebSocket protocol once, use everywhere:
/// - WebSocket handlers (runtime)
/// - API documentation (compile-time)
/// - Client SDKs (generated)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum ApiMessage {
    /// Ping message for connection health checks
    #[serde(rename = "ping")]
    #[asyncapi(summary = "Ping", description = "Keep-alive ping message")]
    Ping { id: u64 },

    /// Pong response to ping
    #[serde(rename = "pong")]
    #[asyncapi(summary = "Pong", description = "Response to ping")]
    Pong { id: u64 },

    /// Data message with payload
    #[serde(rename = "data")]
    #[asyncapi(summary = "Data", description = "Generic data message")]
    Data { payload: String },
}

/// Complete API specification
#[allow(clippy::duplicated_attributes)] // False positive - different operations can reference same channel
#[derive(AsyncApi)]
#[asyncapi(
    title = "Example WebSocket API",
    version = "1.0.0",
    description = "Framework integration example"
)]
#[asyncapi_server(name = "local", host = "localhost:8080", protocol = "ws")]
#[asyncapi_channel(name = "main", address = "/ws")]
#[asyncapi_operation(name = "send", action = "send", channel = "main")]
#[asyncapi_operation(name = "receive", action = "receive", channel = "main")]
struct ExampleApi;

fn main() {
    println!("📚 Framework Integration Guide\n");
    println!("================================================================================\n");

    // Generate spec
    let spec = ExampleApi::asyncapi_spec();
    let messages = ApiMessage::asyncapi_messages();

    println!("## Overview\n");
    println!("asyncapi-rust works with any framework that handles WebSocket connections.");
    println!("The key is to use the same message types in both your handlers and docs.\n");

    println!("## Message Types\n");
    println!("Define your WebSocket protocol once:\n");
    println!("```rust");
    println!("#[derive(Serialize, Deserialize, JsonSchema, ToAsyncApiMessage)]");
    println!("#[serde(tag = \"type\")]");
    println!("pub enum ApiMessage {{");
    println!("    #[serde(rename = \"ping\")]");
    println!("    Ping {{ id: u64 }},");
    println!("    // ... more variants");
    println!("}}");
    println!("```\n");

    println!("## Integration Patterns\n");

    println!("### 1. actix-web + actix-ws\n");
    println!("```rust");
    println!("use actix_ws::Message;");
    println!();
    println!("async fn websocket_handler(");
    println!("    req: HttpRequest,");
    println!("    stream: web::Payload,");
    println!(") -> Result<HttpResponse, Error> {{");
    println!("    let (response, session, mut msg_stream) = actix_ws::handle(&req, stream)?;");
    println!();
    println!("    actix_web::rt::spawn(async move {{");
    println!("        while let Some(Ok(msg)) = msg_stream.next().await {{");
    println!("            if let Message::Text(text) = msg {{");
    println!("                // Parse using your AsyncAPI message type");
    println!("                match serde_json::from_str::<ApiMessage>(&text) {{");
    println!("                    Ok(api_msg) => handle_message(session.clone(), api_msg).await,");
    println!("                    Err(e) => eprintln!(\"Parse error: {{}}\", e),");
    println!("                }}");
    println!("            }}");
    println!("        }}");
    println!("    }});");
    println!();
    println!("    Ok(response)");
    println!("}}");
    println!("```\n");

    println!("### 2. axum\n");
    println!("```rust");
    println!("use axum::{{");
    println!("    extract::ws::{{WebSocket, WebSocketUpgrade, Message}},");
    println!("    response::Response,");
    println!("}};");
    println!();
    println!("async fn websocket_handler(ws: WebSocketUpgrade) -> Response {{");
    println!("    ws.on_upgrade(handle_socket)");
    println!("}}");
    println!();
    println!("async fn handle_socket(mut socket: WebSocket) {{");
    println!("    while let Some(Ok(msg)) = socket.recv().await {{");
    println!("        if let Message::Text(text) = msg {{");
    println!("            // Parse using your AsyncAPI message type");
    println!("            match serde_json::from_str::<ApiMessage>(&text) {{");
    println!("                Ok(api_msg) => {{");
    println!("                    let response = handle_message(api_msg).await;");
    println!("                    let json = serde_json::to_string(&response).unwrap();");
    println!("                    let _ = socket.send(Message::Text(json)).await;");
    println!("                }}");
    println!("                Err(e) => eprintln!(\"Parse error: {{}}\", e),");
    println!("            }}");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("```\n");

    println!("### 3. tungstenite (low-level)\n");
    println!("```rust");
    println!("use tungstenite::{{Message, WebSocket}};");
    println!();
    println!("fn handle_connection(mut socket: WebSocket<TcpStream>) {{");
    println!("    loop {{");
    println!("        match socket.read() {{");
    println!("            Ok(Message::Text(text)) => {{");
    println!("                // Parse using your AsyncAPI message type");
    println!("                if let Ok(api_msg) = serde_json::from_str::<ApiMessage>(&text) {{");
    println!("                    let response = handle_message(api_msg);");
    println!("                    let json = serde_json::to_string(&response).unwrap();");
    println!("                    socket.send(Message::Text(json)).ok();");
    println!("                }}");
    println!("            }}");
    println!("            _ => break,");
    println!("        }}");
    println!("    }}");
    println!("}}");
    println!("```\n");

    println!("## Generating Documentation\n");
    println!("```rust");
    println!("// In your binary or documentation tool:");
    println!("fn main() {{");
    println!("    let spec = ExampleApi::asyncapi_spec();");
    println!("    let json = serde_json::to_string_pretty(&spec).unwrap();");
    println!("    std::fs::write(\"docs/asyncapi.json\", json).unwrap();");
    println!("}}");
    println!("```\n");

    println!("## Best Practices\n");
    println!("1. **Single source of truth**: Define message types once");
    println!("2. **Compile-time validation**: Use strong typing everywhere");
    println!("3. **Runtime parsing**: Let serde handle JSON parsing");
    println!("4. **Error handling**: Always handle parse errors gracefully");
    println!("5. **Documentation**: Keep AsyncAPI spec updated automatically");
    println!("6. **Versioning**: Update spec version with breaking changes\n");

    println!("## Generated Specification\n");
    println!("API: {} v{}", spec.info.title, spec.info.version);
    println!("Messages: {}", messages.len());
    println!();

    let spec_json = serde_json::to_string_pretty(&spec).unwrap();
    println!("```json");
    println!("{}", spec_json);
    println!("```\n");

    println!("================================================================================");
    println!("\n✅ Integration complete! Your WebSocket API is documented and type-safe.");
}
