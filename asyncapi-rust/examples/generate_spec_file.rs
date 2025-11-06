//! Example: Generate AsyncAPI spec file
//!
//! This example demonstrates how to create a standalone binary that
//! generates an AsyncAPI specification file. This approach is useful for:
//! - Committing specs to git for version control
//! - CI/CD workflows that need the spec file
//! - Integration with API documentation tools
//! - Including specs in rustdoc via include_str!
//!
//! ## Usage
//!
//! Run this example to generate the spec file:
//! ```bash
//! cargo run --example generate_spec_file
//! ```
//!
//! ## Project Setup
//!
//! In your own project, create a binary at `bin/generate-asyncapi.rs`:
//! ```text
//! my-project/
//! ├── src/
//! │   └── lib.rs  (contains your API definitions)
//! └── bin/
//!     └── generate-asyncapi.rs  (generates the spec)
//! ```

use asyncapi_rust::{AsyncApi, AsyncApiSpec};
use std::fs;
use std::path::Path;

/// Example API specification
#[derive(AsyncApi)]
#[asyncapi(
    title = "Example WebSocket API",
    version = "1.0.0",
    description = "A real-time API for demonstration purposes"
)]
struct ExampleApi;

fn main() {
    println!("🚀 Generating AsyncAPI specification file...\n");

    // Generate the spec
    let spec: AsyncApiSpec = ExampleApi::asyncapi_spec();

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&spec).expect("Failed to serialize AsyncAPI spec");

    // Create output directory if it doesn't exist
    let output_dir = Path::new("target/asyncapi");
    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    // Write to file
    let output_path = output_dir.join("asyncapi.json");
    fs::write(&output_path, &json).expect("Failed to write spec file");

    println!("✅ Generated: {}", output_path.display());
    println!("\n📄 Specification preview:");
    println!("{}", json);

    println!("\n💡 Usage Tips:");
    println!("   • Commit this file to git for version tracking");
    println!("   • Use in CI/CD: cargo run --bin generate-asyncapi");
    println!("   • Include in rustdoc: #[doc = include_str!(\"path/to/asyncapi.json\")]");
    println!("   • Validate with AsyncAPI tools: asyncapi validate asyncapi.json");
}
