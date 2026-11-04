//! Command surface extracted from Jet. Browser commands keep their behavior.
use anyhow::{Context, Result};
use clap::{Arg, ArgAction, ArgMatches, Command};
use std::path::{Path, PathBuf};

pub fn bb_command() -> Command {
    Command::new("bb")
        .about("Agent-first Browser Bridge commands")
        .subcommand(
            Command::new("launch")
                .about("Launch headless Chromium as a detached Browser Bridge session")
                .arg(
                    Arg::new("url")
                        .required(true)
                        .help("URL to open (typically the `jet serve --wasm` address)"),
                ),
        )
        .subcommand(
            Command::new("shutdown").about("Request the active Browser Bridge session to close"),
        )
        .subcommand(
            Command::new("tree")
                .about("Print the element / layout / fiber tree from the attached session")
                .arg(
                    Arg::new("which")
                        .default_value("element")
                        .help("element | layout | fiber"),
                ),
        )
        .subcommand(
            Command::new("pick")
                .about("Arm a one-shot click listener; next click on the canvas prints the hit-tested node")
                .arg(
                    Arg::new("timeout")
                        .long("timeout")
                        .default_value("30")
                        .help("Seconds to wait for the click"),
                ),
        )
        .subcommand(
            Command::new("hooks")
                .about("Print hook values for a fiber")
                .arg(Arg::new("fiber-id").required(true)),
        )
        .subcommand(
            Command::new("highlight")
                .about("Overlay a red rect on the given layout-node index (or clear when --clear)")
                .arg(Arg::new("index").required(false))
                .arg(
                    Arg::new("clear")
                        .long("clear")
                        .action(ArgAction::SetTrue)
                        .help("Clear the current highlight"),
                ),
        )
        .subcommand(Command::new("frame").about("Dump the last-frame paint ops"))
        .subcommand(
            Command::new("perf")
                .about("Print a compact runtime performance snapshot without full capture"),
        )
        .subcommand(
            Command::new("mouse")
                .about("Dispatch one CDP mouse event into the attached Browser Bridge session")
                .arg(
                    Arg::new("type")
                        .required(true)
                        .value_parser(["mouseMoved", "mousePressed", "mouseReleased"])
                        .help("CDP mouse event type"),
                )
                .arg(Arg::new("x").required(true).help("Viewport CSS-pixel x coordinate"))
                .arg(Arg::new("y").required(true).help("Viewport CSS-pixel y coordinate"))
                .arg(
                    Arg::new("button")
                        .long("button")
                        .value_parser(["left", "right", "middle", "none"])
                        .help("Mouse button for pressed/released events"),
                )
                .arg(
                    Arg::new("buttons")
                        .long("buttons")
                        .help("CDP buttons bitfield, e.g. 1 while dragging with the left button"),
                )
                .arg(
                    Arg::new("click-count")
                        .long("click-count")
                        .help("CDP clickCount for pressed/released events"),
                ),
        )
        .subcommand(
            Command::new("drag")
                .about("Drag between two viewport coordinates using CDP mouse events")
                .arg(Arg::new("from-x").required(true).help("Start x coordinate"))
                .arg(Arg::new("from-y").required(true).help("Start y coordinate"))
                .arg(Arg::new("to-x").required(true).help("End x coordinate"))
                .arg(Arg::new("to-y").required(true).help("End y coordinate"))
                .arg(
                    Arg::new("steps")
                        .long("steps")
                        .default_value("8")
                        .help("Interpolated mouseMoved events between press and release"),
                ),
        )
        .subcommand(
            Command::new("wheel")
                .about("Dispatch one CDP mouse wheel event into the attached Browser Bridge session")
                .arg(Arg::new("x").required(true).help("Viewport CSS-pixel x coordinate"))
                .arg(Arg::new("y").required(true).help("Viewport CSS-pixel y coordinate"))
                .arg(
                    Arg::new("delta-x")
                        .long("delta-x")
                        .default_value("0")
                        .help("Horizontal wheel delta in CSS pixels"),
                )
                .arg(
                    Arg::new("delta-y")
                        .long("delta-y")
                        .default_value("0")
                        .help("Vertical wheel delta in CSS pixels"),
                ),
        )
        .subcommand(
            Command::new("key")
                .about("Press one key in the attached Browser Bridge session using CDP key events")
                .arg(Arg::new("key").required(true).help("Key value such as c, Enter, or ArrowDown"))
                .arg(
                    Arg::new("ctrl")
                        .long("ctrl")
                        .action(ArgAction::SetTrue)
                        .help("Hold Control while pressing the key"),
                )
                .arg(
                    Arg::new("meta")
                        .long("meta")
                        .action(ArgAction::SetTrue)
                        .help("Hold Meta/Command while pressing the key"),
                )
                .arg(
                    Arg::new("shift")
                        .long("shift")
                        .action(ArgAction::SetTrue)
                        .help("Hold Shift while pressing the key"),
                )
                .arg(
                    Arg::new("alt")
                        .long("alt")
                        .action(ArgAction::SetTrue)
                        .help("Hold Alt while pressing the key"),
                ),
        )
        .subcommand(
            Command::new("capture")
                .about("Print a parity-ready JSON observation bundle from an attached wasm or DOM browser session")
                .arg(
                    Arg::new("surface")
                        .long("surface")
                        .default_value("wasm")
                        .value_parser(["wasm", "dom"])
                        .help("Observation surface to capture: wasm debug bridge or live DOM"),
                )
                .arg(
                    Arg::new("root-selector")
                        .long("root-selector")
                        .default_value("body")
                        .help("DOM root selector for --surface dom"),
                )
                .arg(
                    Arg::new("out")
                        .long("out")
                        .short('o')
                        .alias("output")
                        .help("Output file (writes to stdout when omitted)"),
                )
                .arg(
                    Arg::new("pretty")
                        .long("pretty")
                        .action(ArgAction::SetTrue)
                        .help("Pretty-print JSON output"),
                )
                .arg(
                    Arg::new("hook")
                        .long("hook")
                        .action(ArgAction::Append)
                        .num_args(1)
                        .help("Fiber id whose hook values should be included; repeats. Defaults to fibers with hooks."),
                ),
        )
        .subcommand(
            Command::new("screenshot")
                .about("Save a PNG of the current canvas")
                .arg(
                    Arg::new("out")
                        .long("out")
                        .short('o')
                        .help("Output file (writes to stdout when omitted)"),
                ),
        )
        .subcommand(
            Command::new("eval")
                .about("Runtime.evaluate escape hatch")
                .arg(Arg::new("expr").required(true)),
        )
        .subcommand(
            Command::new("mcp")
                .about("Serve the Browser Bridge as an MCP stdio server for agents"),
        )
        .subcommand(
            Command::new("snapshot")
                .about("Print a ref-annotated semantic snapshot of the live DOM; refs (e1, e2, …) feed click/fill/type/hover/select")
                .arg(
                    Arg::new("json")
                        .long("json")
                        .action(ArgAction::SetTrue)
                        .help("Print the full JSON payload instead of the snapshot text"),
                ),
        )
        .subcommand(
            Command::new("click")
                .about("Click an element by snapshot ref (e12) or selector (CSS, text=…, role=…)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector"))
                .arg(
                    Arg::new("dblclick")
                        .long("dblclick")
                        .action(ArgAction::SetTrue)
                        .help("Double-click instead of single click"),
                ),
        )
        .subcommand(
            Command::new("fill")
                .about("Replace an input/textarea value (native setter + input/change events)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector"))
                .arg(Arg::new("text").required(true).help("New value")),
        )
        .subcommand(
            Command::new("type")
                .about("Focus an element and type text through the real CDP input pipeline (appends; use fill to replace)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector"))
                .arg(Arg::new("text").required(true).help("Text to type")),
        )
        .subcommand(
            Command::new("hover")
                .about("Hover an element (mouseenter + mousemove at its center)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector")),
        )
        .subcommand(
            Command::new("select")
                .about("Choose a <select> option by value or label")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector"))
                .arg(Arg::new("option").required(true).help("Option value or label")),
        )
        .subcommand(
            Command::new("check")
                .about("Check a checkbox (idempotent)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector")),
        )
        .subcommand(
            Command::new("uncheck")
                .about("Uncheck a checkbox (idempotent)")
                .arg(Arg::new("target").required(true).help("Snapshot ref (e12) or selector")),
        )
        .subcommand(
            Command::new("goto")
                .about("Navigate the attached session to a URL and wait for load")
                .arg(Arg::new("url").required(true)),
        )
        .subcommand(Command::new("back").about("Go back one entry in session history"))
        .subcommand(Command::new("forward").about("Go forward one entry in session history"))
        .subcommand(Command::new("reload").about("Reload the current document"))
        .subcommand(
            Command::new("resize")
                .about("Resize the viewport (CDP device-metrics override)")
                .arg(Arg::new("width").required(true).help("Viewport width in CSS pixels"))
                .arg(Arg::new("height").required(true).help("Viewport height in CSS pixels")),
        )
        .subcommand(
            Command::new("wait")
                .about("Wait for a selector to attach, text to appear, or a fixed delay")
                .arg(
                    Arg::new("selector")
                        .long("selector")
                        .help("Selector (CSS, text=…, role=…) that must attach"),
                )
                .arg(Arg::new("text").long("text").help("Visible text that must appear"))
                .arg(
                    Arg::new("ms")
                        .long("ms")
                        .help("Sleep this many milliseconds instead of polling"),
                )
                .arg(
                    Arg::new("timeout")
                        .long("timeout")
                        .default_value("10000")
                        .help("Polling budget in milliseconds"),
                ),
        )
        .subcommand(
            Command::new("console")
                .about("Print console messages, page errors, and unhandled rejections captured since launch")
                .arg(
                    Arg::new("level")
                        .long("level")
                        .value_parser(["log", "info", "warn", "error", "debug"])
                        .help("Only entries of this level"),
                )
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .default_value("100")
                        .help("Most-recent entry cap"),
                )
                .arg(
                    Arg::new("clear")
                        .long("clear")
                        .action(ArgAction::SetTrue)
                        .help("Drain the buffer after reading"),
                ),
        )
        .subcommand(
            Command::new("requests")
                .about("Print fetch/XHR activity captured since launch")
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .default_value("100")
                        .help("Most-recent entry cap"),
                )
                .arg(
                    Arg::new("clear")
                        .long("clear")
                        .action(ArgAction::SetTrue)
                        .help("Drain the buffer after reading"),
                ),
        )
        .subcommand(
            Command::new("debug")
                .about("Open a foreground Browser Bridge session for human inspection")
                .arg(Arg::new("url").required(true)),
        )
        .subcommand(
            Command::new("tsx")
                .about("Print TSX source locations for each component (reads dist/tsx-source-map.json — no live browser needed)")
                .arg(
                    Arg::new("filter")
                        .help("Only print components whose name contains this substring"),
                ),
        )
}

pub fn browser_command() -> Command {
    Command::new("browser")
                .about("Browser Bridge management + jet-wasm debugging commands")
                .subcommand(
                    Command::new("install")
                        .about("Download and cache a pinned browser binary")
                        .arg(
                            Arg::new("target")
                                .help("Browser to install (only 'chromium' is supported)")
                                .default_value("chromium"),
                        )
                        .arg(
                            Arg::new("revision")
                                .long("revision")
                                .short('r')
                                .default_value(crate::browser::DEFAULT_CHROMIUM_REVISION)
                                .help("Chromium snapshot revision number"),
                        )
                        .arg(
                            Arg::new("cache-dir")
                                .long("cache-dir")
                                .short('d')
                                .help("Root directory for cached browser installs (default: ~/.jet/browsers)"),
                        ),
                )
                .subcommand(
                    Command::new("launch")
                        .about("Launch Chromium, navigate to URL, and hold a foreground CDP session for other `jet browser` commands to reuse")
                        .arg(Arg::new("url").required(true).help("URL to open (typically the `jet dev --wasm --debug` address)")),
                )
                .subcommand(
                    Command::new("shutdown")
                        .about("Request the active `jet browser launch` session to close"),
                )
                .subcommand(
                    Command::new("tree")
                        .about("Print the element / layout / fiber tree from the attached session")
                        .arg(
                            Arg::new("which")
                                .default_value("element")
                                .help("element | layout | fiber"),
                        ),
                )
                .subcommand(
                    Command::new("pick")
                        .about("Arm a one-shot click listener; next click on the canvas prints the hit-tested node")
                        .arg(
                            Arg::new("timeout")
                                .long("timeout")
                                .default_value("30")
                                .help("Seconds to wait for the click"),
                        ),
                )
                .subcommand(
                    Command::new("hooks")
                        .about("Print hook values for a fiber")
                        .arg(Arg::new("fiber-id").required(true)),
                )
                .subcommand(
                    Command::new("highlight")
                        .about("Overlay a red rect on the given layout-node index (or clear when --clear)")
                        .arg(Arg::new("index").required(false))
                        .arg(
                            Arg::new("clear")
                                .long("clear")
                                .action(ArgAction::SetTrue)
                                .help("Clear the current highlight"),
                        ),
                )
                .subcommand(
                    Command::new("frame")
                        .about("Dump the last-frame paint ops"),
                )
                .subcommand(
                    Command::new("perf")
                        .about("Print a compact runtime performance snapshot without full capture"),
                )
                .subcommand(
                    Command::new("mouse")
                        .about("Dispatch one CDP mouse event into the attached Jet browser session")
                        .arg(
                            Arg::new("type")
                                .required(true)
                                .value_parser(["mouseMoved", "mousePressed", "mouseReleased"])
                                .help("CDP mouse event type"),
                        )
                        .arg(Arg::new("x").required(true).help("Viewport CSS-pixel x coordinate"))
                        .arg(Arg::new("y").required(true).help("Viewport CSS-pixel y coordinate"))
                        .arg(
                            Arg::new("button")
                                .long("button")
                                .value_parser(["left", "right", "middle", "none"])
                                .help("Mouse button for pressed/released events"),
                        )
                        .arg(
                            Arg::new("buttons")
                                .long("buttons")
                                .help("CDP buttons bitfield, e.g. 1 while dragging with the left button"),
                        )
                        .arg(
                            Arg::new("click-count")
                                .long("click-count")
                                .help("CDP clickCount for pressed/released events"),
                        ),
                )
                .subcommand(
                    Command::new("drag")
                        .about("Drag between two viewport coordinates using CDP mouse events")
                        .arg(Arg::new("from-x").required(true).help("Start x coordinate"))
                        .arg(Arg::new("from-y").required(true).help("Start y coordinate"))
                        .arg(Arg::new("to-x").required(true).help("End x coordinate"))
                        .arg(Arg::new("to-y").required(true).help("End y coordinate"))
                        .arg(
                            Arg::new("steps")
                                .long("steps")
                                .default_value("8")
                                .help("Interpolated mouseMoved events between press and release"),
                        ),
                )
                .subcommand(
                    Command::new("wheel")
                        .about("Dispatch one CDP mouse wheel event into the attached Jet browser session")
                        .arg(Arg::new("x").required(true).help("Viewport CSS-pixel x coordinate"))
                        .arg(Arg::new("y").required(true).help("Viewport CSS-pixel y coordinate"))
                        .arg(
                            Arg::new("delta-x")
                                .long("delta-x")
                                .default_value("0")
                                .help("Horizontal wheel delta in CSS pixels"),
                        )
                        .arg(
                            Arg::new("delta-y")
                                .long("delta-y")
                                .default_value("0")
                                .help("Vertical wheel delta in CSS pixels"),
                        ),
                )
                .subcommand(
                    Command::new("key")
                        .about("Press one key in the attached Jet browser session using CDP key events")
                        .arg(Arg::new("key").required(true).help("Key value such as c, Enter, or ArrowDown"))
                        .arg(
                            Arg::new("ctrl")
                                .long("ctrl")
                                .action(ArgAction::SetTrue)
                                .help("Hold Control while pressing the key"),
                        )
                        .arg(
                            Arg::new("meta")
                                .long("meta")
                                .action(ArgAction::SetTrue)
                                .help("Hold Meta/Command while pressing the key"),
                        )
                        .arg(
                            Arg::new("shift")
                                .long("shift")
                                .action(ArgAction::SetTrue)
                                .help("Hold Shift while pressing the key"),
                        )
                        .arg(
                            Arg::new("alt")
                                .long("alt")
                                .action(ArgAction::SetTrue)
                                .help("Hold Alt while pressing the key"),
                        ),
                )
                .subcommand(
                    Command::new("capture")
                        .about("Print a parity-ready JSON observation bundle from an attached wasm or DOM browser session")
                        .arg(
                            Arg::new("surface")
                                .long("surface")
                                .default_value("wasm")
                                .value_parser(["wasm", "dom"])
                                .help("Observation surface to capture: wasm debug bridge or live DOM"),
                        )
                        .arg(
                            Arg::new("root-selector")
                                .long("root-selector")
                                .default_value("body")
                                .help("DOM root selector for --surface dom"),
                        )
                        .arg(
                            Arg::new("out")
                                .long("out")
                                .short('o')
                                .alias("output")
                                .help("Output file (writes to stdout when omitted)"),
                        )
                        .arg(
                            Arg::new("pretty")
                                .long("pretty")
                                .action(ArgAction::SetTrue)
                                .help("Pretty-print JSON output"),
                        )
                        .arg(
                            Arg::new("hook")
                                .long("hook")
                                .action(ArgAction::Append)
                                .num_args(1)
                                .help("Fiber id whose hook values should be included; repeats. Defaults to fibers with hooks."),
                        ),
                )
                .subcommand(
                    Command::new("screenshot")
                        .about("Save a PNG of the current canvas")
                        .arg(
                            Arg::new("out")
                                .long("out")
                                .short('o')
                                .help("Output file (writes to stdout when omitted)"),
                        ),
                )
                .subcommand(
                    Command::new("eval")
                        .about("Runtime.evaluate escape hatch")
                        .arg(Arg::new("expr").required(true)),
                )
                .subcommand(
                    Command::new("debug")
                        .about("Open a foreground Browser Bridge session for human inspection")
                        .arg(Arg::new("url").required(true)),
                )
                .subcommand(
                    Command::new("tsx")
                        .about("Print TSX source locations for each component (reads dist/tsx-source-map.json — no live browser needed)")
                        .arg(
                            Arg::new("filter")
                                .help("Only print components whose name contains this substring"),
                        ),
                )
}

pub fn command() -> Command {
    Command::new("jet-bridge")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Browser and application automation through CLI and MCP")
        .arg(
            Arg::new("root")
                .long("root")
                .global(true)
                .default_value(".")
                .help("Project directory for the bridge session"),
        )
        .subcommand(bb_command())
        .subcommand(browser_command())
        .subcommand(Command::new("mcp").about("Serve the browser bridge over MCP stdio"))
}
pub async fn execute(matches: &ArgMatches) -> Result<()> {
    let root = PathBuf::from(
        matches
            .get_one::<String>("root")
            .expect("root has a default"),
    );
    match matches.subcommand() {
        Some(("mcp", _)) => crate::browser_cli::mcp::serve(&root).await,
        Some((name @ ("bb" | "browser"), args)) => run_browser(name, args, &root).await,
        _ => anyhow::bail!("Choose browser, bb, or mcp; use --help for commands"),
    }
}

pub async fn run_browser(browser_cmd: &str, bm: &ArgMatches, root_dir: &Path) -> Result<()> {
    match bm.subcommand() {
        Some(("install", im)) => {
            let target = im
                .get_one::<String>("target")
                .map(|s| s.as_str())
                .unwrap_or("chromium");

            if target != "chromium" {
                anyhow::bail!(
                    "{} is not yet supported; only chromium is available",
                    target
                );
            }

            let revision = im
                .get_one::<String>("revision")
                .cloned()
                .unwrap_or_else(|| crate::browser::DEFAULT_CHROMIUM_REVISION.to_string());

            // Resolve cache root: --cache-dir flag > ~/.jet/browsers/
            let cache_root: PathBuf = if let Some(dir) = im.get_one::<String>("cache-dir") {
                PathBuf::from(dir)
            } else {
                dirs::home_dir()
                    .context("Could not determine home directory")?
                    .join(".jet")
                    .join("browsers")
            };

            let binary_path =
                crate::browser::install::install_chromium(&revision, &cache_root).await?;

            // Already-installed vs freshly-installed message is transparent to
            // the caller: both cases print the final path.
            println!(
                "Installed chromium {} -> {}",
                revision,
                binary_path.display()
            );
            Ok(())
        }
        Some(("launch", lm)) => {
            let url = lm.get_one::<String>("url").expect("url required");
            if browser_cmd == "bb" {
                crate::browser_cli::launch_detached(root_dir, url).await
            } else {
                crate::browser_cli::launch_foreground(root_dir, url).await
            }
        }
        Some(("debug", lm)) => {
            let url = lm.get_one::<String>("url").expect("url required");
            crate::browser_cli::launch_foreground(root_dir, url).await
        }
        Some(("mcp", _)) => crate::browser_cli::mcp::serve(root_dir).await,
        Some(("shutdown", _)) => crate::browser_cli::shutdown(root_dir).await,
        Some(("tree", tm)) => {
            let which = tm
                .get_one::<String>("which")
                .cloned()
                .unwrap_or_else(|| "element".to_string());
            crate::browser_cli::tree(root_dir, &which).await
        }
        Some(("pick", pm)) => {
            let timeout = pm
                .get_one::<String>("timeout")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--timeout", s))
                .unwrap_or(30);
            crate::browser_cli::pick(root_dir, timeout).await
        }
        Some(("hooks", hm)) => {
            // GH #3651 — converted from `.parse::<u64>().ok()` to
            // `parse_cli_numeric_flag` so a malformed --fiber-id emits a
            // tagged warn via `jet::cli::flags` before the caller's
            // `.context()` fires. Sibling cleanup of the three sites
            // GH #3548 already converted (--port, --port, --timeout).
            let id = hm
                .get_one::<String>("fiber-id")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--fiber-id", s))
                .context("fiber-id must be a number")?;
            crate::browser_cli::hooks(root_dir, id).await
        }
        Some(("highlight", hm)) => {
            let index = if hm.get_flag("clear") {
                None
            } else {
                // GH #3651 — same conversion for --index. `usize` already
                // implements `FromStr<Err = ParseIntError>` so the
                // existing helper covers it without an API change.
                Some(
                    hm.get_one::<String>("index")
                        .and_then(|s| parse_cli_numeric_flag::<usize>("--index", s))
                        .context("highlight requires <index> or --clear")?,
                )
            };
            crate::browser_cli::highlight(root_dir, index).await
        }
        Some(("frame", _)) => crate::browser_cli::frame(root_dir).await,
        Some(("perf", _)) => crate::browser_cli::perf(root_dir).await,
        Some(("mouse", mm)) => {
            let event_type = mm.get_one::<String>("type").expect("type required");
            let x = mm
                .get_one::<String>("x")
                .expect("x required")
                .parse::<f64>()
                .context("x must be a number")?;
            let y = mm
                .get_one::<String>("y")
                .expect("y required")
                .parse::<f64>()
                .context("y must be a number")?;
            let button = mm.get_one::<String>("button").map(String::as_str);
            let buttons = mm
                .get_one::<String>("buttons")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--buttons", s));
            let click_count = mm
                .get_one::<String>("click-count")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--click-count", s));
            crate::browser_cli::mouse(root_dir, event_type, x, y, button, buttons, click_count)
                .await
        }
        Some(("drag", dm)) => {
            let from_x = dm
                .get_one::<String>("from-x")
                .expect("from-x required")
                .parse::<f64>()
                .context("from-x must be a number")?;
            let from_y = dm
                .get_one::<String>("from-y")
                .expect("from-y required")
                .parse::<f64>()
                .context("from-y must be a number")?;
            let to_x = dm
                .get_one::<String>("to-x")
                .expect("to-x required")
                .parse::<f64>()
                .context("to-x must be a number")?;
            let to_y = dm
                .get_one::<String>("to-y")
                .expect("to-y required")
                .parse::<f64>()
                .context("to-y must be a number")?;
            let steps = dm
                .get_one::<String>("steps")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--steps", s))
                .unwrap_or(8);
            crate::browser_cli::drag(root_dir, from_x, from_y, to_x, to_y, steps).await
        }
        Some(("wheel", wm)) => {
            let x = wm
                .get_one::<String>("x")
                .expect("x required")
                .parse::<f64>()
                .context("x must be a number")?;
            let y = wm
                .get_one::<String>("y")
                .expect("y required")
                .parse::<f64>()
                .context("y must be a number")?;
            let delta_x = wm
                .get_one::<String>("delta-x")
                .expect("delta-x has default")
                .parse::<f64>()
                .context("--delta-x must be a number")?;
            let delta_y = wm
                .get_one::<String>("delta-y")
                .expect("delta-y has default")
                .parse::<f64>()
                .context("--delta-y must be a number")?;
            crate::browser_cli::wheel(root_dir, x, y, delta_x, delta_y).await
        }
        Some(("key", km)) => {
            let key = km.get_one::<String>("key").expect("key required");
            let mut modifiers = 0;
            if km.get_flag("alt") {
                modifiers |= 1;
            }
            if km.get_flag("ctrl") {
                modifiers |= 2;
            }
            if km.get_flag("meta") {
                modifiers |= 4;
            }
            if km.get_flag("shift") {
                modifiers |= 8;
            }
            crate::browser_cli::key(root_dir, key, modifiers).await
        }
        Some(("capture", cm)) => {
            let out = cm.get_one::<String>("out").map(PathBuf::from);
            let surface = cm
                .get_one::<String>("surface")
                .map(String::as_str)
                .unwrap_or("wasm");
            let root_selector = cm
                .get_one::<String>("root-selector")
                .map(String::as_str)
                .unwrap_or("body");
            let hooks = cm
                .get_many::<String>("hook")
                .into_iter()
                .flatten()
                .map(|raw| {
                    parse_cli_numeric_flag::<u64>("--hook", raw)
                        .context("--hook must be a fiber id")
                })
                .collect::<Result<Vec<_>>>()?;
            crate::browser_cli::capture(
                root_dir,
                surface,
                root_selector,
                &hooks,
                cm.get_flag("pretty"),
                out.as_deref(),
            )
            .await
        }
        Some(("screenshot", sm)) => {
            let out = sm.get_one::<String>("out").map(PathBuf::from);
            crate::browser_cli::screenshot(root_dir, out.as_deref()).await
        }
        Some(("eval", em)) => {
            let expr = em.get_one::<String>("expr").expect("expr required");
            crate::browser_cli::eval(root_dir, expr).await
        }
        Some(("tsx", tm)) => {
            let filter = tm.get_one::<String>("filter").map(|s| s.as_str());
            crate::browser_cli::tsx(root_dir, filter)
        }
        Some(("snapshot", sm)) => {
            let v = crate::browser_cli::interact::snapshot(root_dir).await?;
            if sm.get_flag("json") {
                println!("{}", serde_json::to_string_pretty(&v)?);
            } else {
                let url = v["url"].as_str().unwrap_or("");
                let title = v["title"].as_str().unwrap_or("");
                println!("# {title} — {url}");
                if v["truncated"].as_bool() == Some(true) {
                    println!("# (truncated at element cap — use a tighter page state)");
                }
                println!("{}", v["snapshot"].as_str().unwrap_or(""));
            }
            Ok(())
        }
        Some((verb @ ("click" | "hover" | "check" | "uncheck"), am)) => {
            let raw = am.get_one::<String>("target").expect("target required");
            let target = crate::browser_cli::interact::parse_target(raw)?;
            let v = match verb {
                "click" => {
                    crate::browser_cli::interact::click(root_dir, &target, am.get_flag("dblclick"))
                        .await?
                }
                "hover" => crate::browser_cli::interact::hover(root_dir, &target).await?,
                "check" => {
                    crate::browser_cli::interact::set_checked(root_dir, &target, true).await?
                }
                _ => crate::browser_cli::interact::set_checked(root_dir, &target, false).await?,
            };
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some((verb @ ("fill" | "type"), am)) => {
            let raw = am.get_one::<String>("target").expect("target required");
            let text = am.get_one::<String>("text").expect("text required");
            let target = crate::browser_cli::interact::parse_target(raw)?;
            let v = if verb == "fill" {
                crate::browser_cli::interact::fill(root_dir, &target, text).await?
            } else {
                crate::browser_cli::interact::type_text(root_dir, &target, text).await?
            };
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("select", am)) => {
            let raw = am.get_one::<String>("target").expect("target required");
            let option = am.get_one::<String>("option").expect("option required");
            let target = crate::browser_cli::interact::parse_target(raw)?;
            let v = crate::browser_cli::interact::select(root_dir, &target, option).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("goto", gm)) => {
            let url = gm.get_one::<String>("url").expect("url required");
            let v = crate::browser_cli::interact::goto(root_dir, url).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("back", _)) => {
            let v = crate::browser_cli::interact::history_step(root_dir, -1).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("forward", _)) => {
            let v = crate::browser_cli::interact::history_step(root_dir, 1).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("reload", _)) => {
            let v = crate::browser_cli::interact::reload(root_dir).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("resize", rm)) => {
            let width = rm
                .get_one::<String>("width")
                .and_then(|s| parse_cli_numeric_flag::<u64>("width", s))
                .context("width must be a number")?;
            let height = rm
                .get_one::<String>("height")
                .and_then(|s| parse_cli_numeric_flag::<u64>("height", s))
                .context("height must be a number")?;
            let v = crate::browser_cli::interact::resize(root_dir, width, height).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("wait", wm)) => {
            let selector = wm.get_one::<String>("selector").map(String::as_str);
            let text = wm.get_one::<String>("text").map(String::as_str);
            let ms = wm
                .get_one::<String>("ms")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--ms", s));
            let timeout = wm
                .get_one::<String>("timeout")
                .and_then(|s| parse_cli_numeric_flag::<u64>("--timeout", s))
                .unwrap_or(10_000);
            let v =
                crate::browser_cli::interact::wait(root_dir, selector, text, ms, timeout).await?;
            println!("{}", serde_json::to_string(&v)?);
            Ok(())
        }
        Some(("console", cm)) => {
            let level = cm.get_one::<String>("level").map(String::as_str);
            let limit = cm
                .get_one::<String>("limit")
                .and_then(|s| parse_cli_numeric_flag::<usize>("--limit", s))
                .unwrap_or(100);
            let v =
                crate::browser_cli::interact::console(root_dir, level, limit, cm.get_flag("clear"))
                    .await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            Ok(())
        }
        Some(("requests", rm)) => {
            let limit = rm
                .get_one::<String>("limit")
                .and_then(|s| parse_cli_numeric_flag::<usize>("--limit", s))
                .unwrap_or(100);
            let v = crate::browser_cli::interact::requests(root_dir, limit, rm.get_flag("clear"))
                .await?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            Ok(())
        }
        _ => {
            anyhow::bail!(
                "Unknown browser subcommand. Try one of: install, launch, shutdown, debug, \
                     tree, pick, hooks, highlight, frame, perf, mouse, drag, key, capture, \
                     screenshot, eval, snapshot, click, fill, type, hover, select, check, \
                     uncheck, goto, back, forward, reload, resize, wait, console, requests."
            )
        }
    }
}
pub(crate) fn format_cli_flag_parse_warn(
    flag: &str,
    value: &str,
    type_name: &str,
    err: &std::num::ParseIntError,
) -> String {
    format!(
        "GH #3548 CLI flag {flag}={value:?} could not be parsed as {type_name}: {err}; the flag will be IGNORED and the built-in default will be used instead. Re-run with a numeric value (e.g. {flag}=8080)."
    )
}

pub(crate) fn parse_cli_numeric_flag<T>(flag: &str, value: &str) -> Option<T>
where
    T: std::str::FromStr<Err = std::num::ParseIntError>,
{
    match value.parse::<T>() {
        Ok(v) => Some(v),
        Err(err) => {
            tracing::warn!(
                target: "jet::cli::flags",
                flag = flag,
                value = value,
                expected = std::any::type_name::<T>(),
                error = %err,
                "{}",
                format_cli_flag_parse_warn(flag, value, std::any::type_name::<T>(), &err)
            );
            None
        }
    }
}
