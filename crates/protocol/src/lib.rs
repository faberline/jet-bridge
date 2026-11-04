//! Shared wire data. This crate has no browser, renderer, CLI or MCP dependency.
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub const VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub version: u32,
    pub id: u64,
    pub command: Command,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "method", rename_all = "snake_case", deny_unknown_fields)]
pub enum Command {
    Capabilities,
    Snapshot,
    Query {
        #[serde(default)]
        id: Option<String>,
        #[serde(default)]
        role: Option<String>,
        #[serde(default)]
        name: Option<String>,
    },
    Activate {
        id: String,
    },
    Fill {
        id: String,
        value: String,
    },
    SetValue {
        id: String,
        value: f64,
    },
    Key {
        key: Key,
        #[serde(default)]
        extend: bool,
        #[serde(default)]
        by_word: bool,
    },
}
impl Command {
    pub fn is_mutating(&self) -> bool {
        !matches!(
            self,
            Self::Capabilities | Self::Snapshot | Self::Query { .. }
        )
    }
}
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    Tab,
    Enter,
    Space,
    Escape,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Backspace,
    Delete,
    SelectAll,
    Undo,
    Redo,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub version: u32,
    pub id: u64,
    #[serde(flatten)]
    pub outcome: Outcome,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum Outcome {
    Ok { result: Value },
    Error { code: String, message: String },
}
impl Response {
    pub fn ok(id: u64, result: Value) -> Self {
        Self {
            version: VERSION,
            id,
            outcome: Outcome::Ok { result },
        }
    }
    pub fn error(id: u64, code: &str, message: impl Into<String>) -> Self {
        Self {
            version: VERSION,
            id,
            outcome: Outcome::Error {
                code: code.into(),
                message: message.into(),
            },
        }
    }
}

/// Opt-in native connection. Do not log or publish its token.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NativeSession {
    pub version: u32,
    pub endpoint: String,
    pub token: String,
    pub pid: u32,
    pub application: String,
}
impl std::fmt::Debug for NativeSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeSession")
            .field("version", &self.version)
            .field("endpoint", &self.endpoint)
            .field("token", &"[redacted]")
            .field("pid", &self.pid)
            .field("application", &self.application)
            .finish()
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthenticatedRequest {
    pub token: String,
    pub request: Request,
}
