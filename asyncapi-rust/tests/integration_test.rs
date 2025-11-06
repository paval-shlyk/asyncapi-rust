use asyncapi_rust::ToAsyncApiMessage;
use serde::{Deserialize, Serialize};

// Test basic enum without serde attributes
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
pub enum BasicMessage {
    Ping,
    Pong,
}

// Test enum with serde tag
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
#[serde(tag = "type")]
pub enum TaggedMessage {
    Echo { text: String },
    Broadcast { room: String, text: String },
}

// Test enum with serde rename on variants
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
#[serde(tag = "message")]
pub enum RenamedMessage {
    #[serde(rename = "user.join")]
    UserJoin { username: String },
    #[serde(rename = "user.leave")]
    UserLeave { username: String },
    #[serde(rename = "chat.message")]
    ChatMessage { username: String, text: String },
}

// Test struct
#[derive(Serialize, Deserialize, ToAsyncApiMessage)]
pub struct SimpleMessage {
    pub id: u64,
    pub text: String,
}

#[test]
fn test_basic_enum_messages() {
    let names = BasicMessage::asyncapi_message_names();
    assert_eq!(names, vec!["Ping", "Pong"]);
    assert_eq!(BasicMessage::asyncapi_message_count(), 2);
    assert_eq!(BasicMessage::asyncapi_tag_field(), None);
}

#[test]
fn test_tagged_enum() {
    let names = TaggedMessage::asyncapi_message_names();
    assert_eq!(names, vec!["Echo", "Broadcast"]);
    assert_eq!(TaggedMessage::asyncapi_message_count(), 2);
    assert_eq!(TaggedMessage::asyncapi_tag_field(), Some("type"));
}

#[test]
fn test_renamed_enum() {
    let names = RenamedMessage::asyncapi_message_names();
    assert_eq!(names, vec!["user.join", "user.leave", "chat.message"]);
    assert_eq!(RenamedMessage::asyncapi_message_count(), 3);
    assert_eq!(RenamedMessage::asyncapi_tag_field(), Some("message"));
}

#[test]
fn test_struct_message() {
    let names = SimpleMessage::asyncapi_message_names();
    assert_eq!(names, vec!["SimpleMessage"]);
    assert_eq!(SimpleMessage::asyncapi_message_count(), 1);
    assert_eq!(SimpleMessage::asyncapi_tag_field(), None);
}
