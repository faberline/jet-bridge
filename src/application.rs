//! One semantic client for native applications and browser-hosted WASM apps.
use anyhow::{bail, ensure, Context, Result};
use jet_bridge_protocol::{
    AuthenticatedRequest, Command, NativeSession, Outcome, Request, Response, VERSION,
};
use serde_json::{json, Value};
pub mod cli;
use std::{
    io::{BufRead, BufReader, Read, Write},
    net::{SocketAddr, TcpStream},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

#[derive(Clone, Debug, Default)]
pub struct Options {
    pub app_session: Option<PathBuf>,
}

pub async fn call(root: &Path, options: &Options, command: Command) -> Result<Value> {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let request = Request {
        version: VERSION,
        id: NEXT.fetch_add(1, Ordering::Relaxed),
        command,
    };
    let id = request.id;
    let response = if let Some(path) = &options.app_session {
        let path = path.clone();
        tokio::task::spawn_blocking(move || native_call(&path, request)).await??
    } else {
        let page = crate::browser_cli::attach(root).await?;
        let available = page
            .evaluate("typeof window.__jet_bridge?.request === 'function'")
            .await?;
        ensure!(
            available == true,
            "this page has no application bridge; enable Surface devtools"
        );
        let value = page
            .evaluate(&format!(
                "window.__jet_bridge.request({})",
                serde_json::to_string(&request)?
            ))
            .await?;
        serde_json::from_value(value).context("invalid WASM application bridge response")?
    };
    ensure!(
        response.version == VERSION && response.id == id,
        "application response version/id mismatch"
    );
    match response.outcome {
        Outcome::Ok { result } => Ok(result),
        Outcome::Error { code, message } => bail!("{code}: {message}"),
    }
}

fn native_call(path: &Path, request: Request) -> Result<Response> {
    let session: NativeSession =
        serde_json::from_slice(&std::fs::read(path).context("reading application session file")?)?;
    ensure!(
        session.version == VERSION,
        "unsupported application session version"
    );
    let address: SocketAddr = session
        .endpoint
        .parse()
        .context("invalid application endpoint")?;
    ensure!(
        address.ip().is_loopback(),
        "application endpoint must use loopback"
    );
    let mut socket = TcpStream::connect_timeout(&address, Duration::from_secs(5))
        .context("connecting to native application")?;
    socket.set_read_timeout(Some(Duration::from_secs(10)))?;
    socket.set_write_timeout(Some(Duration::from_secs(5)))?;
    let mut bytes = serde_json::to_vec(&AuthenticatedRequest {
        token: session.token,
        request,
    })?;
    bytes.push(b'\n');
    socket.write_all(&bytes)?;
    let mut response = Vec::new();
    const LIMIT: u64 = 8 * 1024 * 1024;
    BufReader::new(socket)
        .take(LIMIT + 1)
        .read_until(b'\n', &mut response)
        .context("application did not reply; a submitted action may already have run")?;
    ensure!(
        response.len() as u64 <= LIMIT && response.last() == Some(&b'\n'),
        "incomplete or oversized application response"
    );
    serde_json::from_slice(&response).context("invalid native application response")
}

pub fn tool_definitions() -> Vec<Value> {
    let id = json!({"type":"string","description":"Stable UI node ID"});
    let tool = |name: &str,
                description: &str,
                properties: Value,
                required: Vec<&str>,
                read_only: bool| {
        json!({
            "name":name,"description":description,
            "inputSchema":{"type":"object","properties":properties,"required":required,"additionalProperties":false},
            "annotations":{"readOnlyHint":read_only}
        })
    };
    vec![
        tool(
            "ui_capabilities",
            "Read the application's semantic bridge capabilities.",
            json!({}),
            vec![],
            true,
        ),
        tool(
            "ui_snapshot",
            "Export the current Rust UI tree, layout and editing state.",
            json!({}),
            vec![],
            true,
        ),
        tool(
            "ui_query",
            "Find current UI nodes by stable ID, role or accessible name.",
            json!({"id":id,"role":{"type":"string"},"name":{"type":"string"}}),
            vec![],
            true,
        ),
        tool(
            "ui_activate",
            "Activate an enabled control through the application's normal input path.",
            json!({"id":id}),
            vec!["id"],
            false,
        ),
        tool(
            "ui_fill",
            "Replace a writable field's value through controlled application state.",
            json!({"id":id,"value":{"type":"string"}}),
            vec!["id", "value"],
            false,
        ),
        tool(
            "ui_set_value",
            "Set a controlled range value.",
            json!({"id":id,"value":{"type":"number"}}),
            vec!["id", "value"],
            false,
        ),
        tool(
            "ui_key",
            "Send a semantic navigation/editing key to the focused control.",
            json!({"key":{"type":"string","enum":KEYS},"extend":{"type":"boolean"},"by_word":{"type":"boolean"}}),
            vec!["key"],
            false,
        ),
    ]
}
pub const KEYS: [&str; 17] = [
    "tab",
    "enter",
    "space",
    "escape",
    "left",
    "right",
    "up",
    "down",
    "home",
    "end",
    "page_up",
    "page_down",
    "backspace",
    "delete",
    "select_all",
    "undo",
    "redo",
];

pub async fn call_tool(
    root: &Path,
    options: &Options,
    name: &str,
    args: &Value,
) -> Result<Vec<Value>> {
    let method = name
        .strip_prefix("ui_")
        .context("unknown application tool")?;
    let mut data = args
        .as_object()
        .context("tool arguments must be an object")?
        .clone();
    ensure!(
        !data.contains_key("method"),
        "method is not a tool argument"
    );
    data.insert("method".into(), method.into());
    let command = serde_json::from_value(Value::Object(data))
        .context("invalid application tool arguments")?;
    let value = call(root, options, command).await?;
    Ok(vec![
        json!({"type":"text","text":serde_json::to_string_pretty(&value)?}),
    ])
}
