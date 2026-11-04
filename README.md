# Jet bridge

A standalone extraction of Jet's browser automation code.
Source and license evidence is in `docs/provenance.json`.

The current main-branch checkpoint is tracked in
[jet-bridge@0.1.0](https://github.com/faberline/jet-bridge/milestone/1)
and [issue #3](https://github.com/faberline/jet-bridge/issues/3).
Hosted browser/SDK gates and transport hardening are tracked in
[jet-bridge@0.2.0](https://github.com/faberline/jet-bridge/milestone/2).
Milestone landing does not publish a crate or release tag.

The initial library contains Chromium transport, page/locator operations,
semantic DOM interaction, session storage, CLI helpers and an MCP server.
It has no dependency on Jet's build tool, Core or Surface.

## Run

Tested with Rust 1.96.1. Use `cap` and two build jobs locally.

```sh
export PATH="$HOME/.cargo/bin:$PATH"
export CARGO_BUILD_JOBS=2
cap cargo build --locked
target/debug/jet-bridge --help
```

The standalone CLI retains both command groups. `bb launch` creates a
detached browser. `browser launch` holds a foreground browser session.
Commands in the same project directory reuse its `.jet/browser-session.json`.

```sh
jet-bridge --root /path/to/project bb launch http://127.0.0.1:8080
jet-bridge --root /path/to/project bb snapshot --json
jet-bridge --root /path/to/project bb fill '#name' '中文橋接'
jet-bridge --root /path/to/project bb shutdown
```

Run `jet-bridge --root /path/to/project mcp` for MCP over stdio.
Diagnostics go to stderr. The imported MCP contract is `2025-06-18`.
The server advertises 27 browser tools. It does not claim the newer MCP
protocol without separate conformance work.

## Verification and limits

The initial extraction passes format, strict Clippy, 166 library cases,
native build, and a real Chromium CLI/MCP journey. Run the latter with:

```sh
CHROME_PATH=/path/to/chrome python3 scripts/live-browser.py
```

The test launches its own browser and local fixture. It checks Chinese
fill events, clicks, shared CLI/MCP state, tool errors, PNG capture and
actual browser exit after shutdown. It uses no Playwright runtime.

Two cross-module tests remain in the Jet consumer. The initial extraction
and protocol/client revision pass Linux CI. The
[Jet consumer cutover](https://github.com/faberline/jet/pull/76) passes its
local browser and WASM journeys. The
[Surface adapters](https://github.com/faberline/surface/pull/3) pass actual
macOS native and Chrome 157 WebGPU CLI/MCP journeys.
See [the application bridge boundary](docs/application-bridge.md).
Full Playwright parity and other native platforms remain unqualified.

## Application protocol and clients

`jet-bridge-protocol` is a small workspace crate. It depends only on Serde
and JSON. A Surface app can use this crate without the browser transport,
CLI or MCP dependencies. Requests and responses use protocol version 1.

The native client reads an explicit private session file. It accepts only
loopback addresses. The WASM client calls `window.__jet_bridge.request`
through the existing browser connection. Both clients validate the response
version and request ID. They do not retry submitted actions after a timeout.

```sh
jet-bridge --app-session /path/to/session.json ui snapshot
jet-bridge --app-session /path/to/session.json ui fill name '中文橋接'
jet-bridge --app-session /path/to/session.json mcp
jet-bridge --root /path/to/project ui query --role TextInput
jet-bridge --root /path/to/project mcp --ui
```

The UI MCP mode exposes seven semantic tools. The default MCP mode retains
the 27 browser tools. These clients require an application adapter. The
Surface adapters live in the Surface repository. Their live checks cover
current DOM/layout, controlled Chinese fill, saving/cancelling, range
values, disabled/removed controls, modal focus, native session cleanup and
actual browser exit. Physical OS IME input remains a separate consumer check.
