#!/usr/bin/env python3
"""Exercise CLI and MCP against an owned local Chromium fixture. No Playwright."""
import http.server
import json
import os
from pathlib import Path
import select
import signal
import subprocess
import tempfile
import threading
import time

ROOT = Path(__file__).resolve().parents[1]
ARTIFACTS = ROOT / "artifacts"
ARTIFACTS.mkdir(exist_ok=True)
BINARY = Path(os.environ.get("JET_BRIDGE_BIN", ROOT / "target/debug/jet-bridge"))
HTML = b"""<!doctype html><meta charset=utf-8><title>Jet bridge fixture</title>
<h1>Bridge form</h1><label>Name<input id=name oninput='window.lastName=this.value'></label>
<button id=add onclick='window.count=(window.count||0)+1;document.querySelector("output").textContent=window.count'>Add</button>
<output>0</output>"""


class Fixture(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header("Content-Type", "text/html; charset=utf-8")
        self.end_headers()
        self.wfile.write(HTML)

    def log_message(self, *_):
        pass


def main():
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Fixture)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    owned_browser_pid = None
    mcp = None
    try:
        with tempfile.TemporaryDirectory(prefix="jet-bridge-live-") as folder, \
             (ARTIFACTS / "live-browser-stderr.log").open("w") as diagnostic:
            prefix = [str(BINARY), "--root", folder]

            def cli(*args):
                result = subprocess.run(prefix + list(args), cwd=ROOT,
                                        stdout=subprocess.PIPE, stderr=diagnostic,
                                        text=True, timeout=35, check=True)
                return result.stdout

            url = f"http://127.0.0.1:{server.server_port}/"
            launch = json.loads(cli("bb", "launch", url))
            owned_browser_pid = launch["pid"]
            snapshot = json.loads(cli("bb", "snapshot", "--json"))
            assert snapshot["ref_count"] >= 2, snapshot

            mcp = subprocess.Popen(prefix + ["mcp"], cwd=ROOT,
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=diagnostic, text=True, bufsize=1)
            sequence = 0

            def request(method, params):
                nonlocal sequence
                sequence += 1
                mcp.stdin.write(json.dumps({"jsonrpc": "2.0", "id": sequence,
                                           "method": method, "params": params}) + "\n")
                mcp.stdin.flush()
                assert select.select([mcp.stdout], [], [], 15)[0], "MCP response timed out"
                response = json.loads(mcp.stdout.readline())
                assert response["id"] == sequence, response
                assert "error" not in response, response
                return response["result"]

            initialized = request("initialize", {
                "protocolVersion": "2025-06-18", "capabilities": {},
                "clientInfo": {"name": "jet-bridge-live-check", "version": "1"}})
            assert initialized["protocolVersion"] == "2025-06-18", initialized
            mcp.stdin.write('{"jsonrpc":"2.0","method":"notifications/initialized"}\n')
            mcp.stdin.flush()
            tools = request("tools/list", {})["tools"]
            names = {tool["name"] for tool in tools}
            assert {"bb_snapshot", "bb_fill", "bb_click", "bb_eval"} <= names

            def tool(name, arguments):
                result = request("tools/call", {"name": name, "arguments": arguments})
                assert not result.get("isError"), result
                return json.loads(result["content"][0]["text"])

            tool("bb_fill", {"selector": "#name", "text": "中文橋接"})
            tool("bb_click", {"selector": "#add"})
            expression = "JSON.stringify({name:document.querySelector('#name').value,lastName:window.lastName,count:window.count})"
            observed = json.loads(tool("bb_eval", {"expression": expression}))
            assert observed == {"name": "中文橋接", "lastName": "中文橋接", "count": 1}, observed
            assert json.loads(json.loads(cli("bb", "eval", expression))) == observed
            rejected = request("tools/call", {"name": "unknown_tool", "arguments": {}})
            assert rejected["isError"] is True, rejected
            cli("bb", "screenshot", "--out", str(ARTIFACTS / "live-browser.png"))
            assert (ARTIFACTS / "live-browser.png").read_bytes().startswith(b"\x89PNG\r\n\x1a\n")
            tool("bb_shutdown", {})
            deadline = time.monotonic() + 10
            while time.monotonic() < deadline:
                try:
                    os.kill(owned_browser_pid, 0)
                except ProcessLookupError:
                    owned_browser_pid = None
                    break
                time.sleep(0.05)
            assert owned_browser_pid is None, "shutdown ACK did not prove browser exit"
            evidence = {"protocol": initialized["protocolVersion"], "tool_count": len(tools),
                        "state": observed, "cli_mcp_share_session": True,
                        "screenshot": "live-browser.png", "owned_browser_exited": True}
            (ARTIFACTS / "live-browser.json").write_text(json.dumps(evidence, ensure_ascii=False, indent=2) + "\n")
            print(json.dumps(evidence, ensure_ascii=False))
    finally:
        if mcp is not None:
            if mcp.stdin and not mcp.stdin.closed:
                mcp.stdin.close()
            try:
                mcp.wait(timeout=3)
            except subprocess.TimeoutExpired:
                mcp.terminate()
                mcp.wait(timeout=3)
        if owned_browser_pid is not None:
            try:
                os.kill(owned_browser_pid, signal.SIGTERM)
            except ProcessLookupError:
                pass
        server.shutdown()
        server.server_close()
        thread.join(timeout=3)


if __name__ == "__main__":
    main()
