Use short, plain sentences in user-facing reports.

Jet bridge is independent of Jet's build tool and Surface's renderer.
Keep browser transport, application protocol and CLI/MCP adapters separate.
Preserve imported public APIs while Jet consumers migrate.
Keep original license and source provenance.
Keep Jet cross-module acceptance checks in the consumer when extracting them.
Do not remove assertions to make a standalone test compile.

Use cap and CARGO_BUILD_JOBS=2 for local Rust builds.
Run format, affected tests and cargo check before reporting a built change.
Run actual local browser/app journeys for automation behavior.
Do not treat skipped browser tests as acceptance.
Control only test browsers and applications created by this task.
