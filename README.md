# Jet bridge

A standalone extraction of Jet's browser automation code.
Source and license evidence is in `docs/provenance.json`.

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

Two cross-module tests remain in the Jet consumer. Jet cutover, hosted CI,
the Surface desktop connection and WASM semantic adapter are still pending.
See [the application bridge boundary](docs/application-bridge.md).
No full Playwright replacement or native app acceptance is claimed.
