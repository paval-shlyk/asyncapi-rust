# asyncapi-rust

> **⚠️ Early Development Notice**
>
> This project is in the early stages of development. Documentation, examples, and some links may be incomplete or broken. The API is not yet stable and may change significantly. Stay tuned for updates!

[![Crates.io](https://img.shields.io/crates/v/asyncapi-rust.svg)](https://crates.io/crates/asyncapi-rust)
[![Documentation](https://docs.rs/asyncapi-rust/badge.svg)](https://docs.rs/asyncapi-rust)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](https://opensource.org/licenses/MIT)

**AsyncAPI 3.0 specification generation for Rust WebSockets and async protocols**

Generate AsyncAPI documentation directly from your Rust code using procedural macros. Similar to how `utoipa` generates OpenAPI specs for REST APIs, `asyncapi-rust` generates AsyncAPI specs for WebSocket and other async protocols.

## Features

- 🦀 **Code-first**: Generate specs from Rust types, not YAML
- ⚡ **Compile-time**: Zero runtime cost, all generation at build time
- 🔒 **Type-safe**: Compile errors if documentation drifts from code
- 🎯 **Familiar**: Follows patterns from `utoipa`, `serde`, and `clap`
- 🌐 **Framework agnostic**: Works with actix-ws, axum, or any serde-compatible types
- 📦 **Binary protocols**: Support for mixed text/binary WebSocket messages (Arrow IPC, Protobuf, etc.)

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
asyncapi-rust = "0.1"
serde = { version = "1.0", features = ["derive"] }
```

Define your WebSocket messages:

```rust
use asyncapi_rust::{AsyncApi, ToAsyncApiMessage};
use serde::{Deserialize, Serialize};

/// Client → Server messages
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum ClientMessage {
    /// Send a chat message
    #[asyncapi(summary = "Send message to chat room")]
    SendMessage { room: String, text: String },

    /// Join a chat room
    JoinRoom { room: String },
}

/// Server → Client messages
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum ServerMessage {
    /// New message from another user
    MessageReceived { room: String, user: String, text: String },

    /// Confirmation of room join
    RoomJoined { room: String },
}

/// Generate the AsyncAPI specification
#[derive(AsyncApi)]
#[asyncapi(
    title = "Chat WebSocket API",
    version = "1.0.0",
    description = "Real-time chat application",
    server(name = "production", url = "wss://chat.example.com/ws"),
)]
struct ChatApi;

fn main() {
    // Get the generated spec
    let spec = ChatApi::asyncapi();

    // Serve it as JSON
    println!("{}", serde_json::to_string_pretty(&spec).unwrap());
}
```

## Motivation

Manually maintaining AsyncAPI specifications is error-prone and time-consuming:

- ❌ Type changes in Rust require manual YAML updates
- ❌ No compile-time validation of documentation accuracy
- ❌ Easy for docs to drift from implementation
- ❌ Repetitive work defining the same types twice

**asyncapi-rust solves this** by generating AsyncAPI specs directly from your Rust types, providing a single source of truth with compile-time guarantees.

## Comparison: Manual vs Generated

**Before (Manual YAML):**
```yaml
# asyncapi.yaml - must keep in sync manually!
components:
  messages:
    SendMessage:
      payload:
        type: object
        properties:
          type: { type: string, const: SendMessage }
          room: { type: string }
          text: { type: string }
```

**After (Generated from Rust):**
```rust
/// Send a chat message
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
#[serde(tag = "type", rename = "SendMessage")]
pub struct SendMessage {
    pub room: String,
    pub text: String,
}
// AsyncAPI YAML generated automatically at compile time!
```

## Supported Frameworks

- ✅ **actix-ws** - Full integration with actix-web WebSocket handlers
- ✅ **axum** - Integration with axum WebSocket routes
- 🔄 **Framework-agnostic** - Works with any serde-compatible message types

## Binary Protocol Support

Document binary WebSocket messages (Arrow IPC, Protobuf, MessagePack):

```rust
/// Binary data stream
#[derive(ToAsyncApiMessage)]
#[asyncapi(
    name = "BinaryData",
    content_type = "application/octet-stream",
    binary = true,
    description = "Binary data payload",
)]
pub struct BinaryData;
```

## Examples

- [Basic Chat](examples/basic-chat/) - Simple text-only WebSocket
- [Binary Streaming](examples/binary-streaming/) - Mixed text/binary protocol
- [actix-web Integration](examples/actix-integration/) - Full actix-ws integration
- [axum Integration](examples/axum-integration/) - Full axum integration

## Documentation

- [API Documentation](https://docs.rs/asyncapi-rust)
- [User Guide](docs/guide.md)
- [Migration from Manual Specs](docs/migration.md)
- [Binary Protocol Support](docs/binary-protocols.md)

## Roadmap

- [x] Core macro implementation
- [x] actix-ws integration
- [x] axum integration
- [x] Binary message support
- [ ] Embedded AsyncAPI UI
- [ ] Additional framework support (tonic/gRPC, Rocket, Warp)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgments

Inspired by:
- [utoipa](https://github.com/juhaku/utoipa) - OpenAPI code generation for Rust
- [AsyncAPI Initiative](https://www.asyncapi.com/) - AsyncAPI specification

---

**Author:** Mark Lilback (mark@lilback.com)
**Repository:** https://github.com/mlilback/asyncapi-rust
