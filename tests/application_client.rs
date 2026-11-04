use jet_bridge::{application, cli};
use jet_bridge_protocol::{AuthenticatedRequest, Command, NativeSession, VERSION};
use serde_json::json;
use std::{
    io::{BufRead, BufReader, Write},
    net::TcpListener,
};

#[tokio::test]
async fn native_client_sends_auth_and_rejects_an_unrelated_reply() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("session.json");
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    std::fs::write(
        &path,
        serde_json::to_vec(&NativeSession {
            version: VERSION,
            endpoint: listener.local_addr().unwrap().to_string(),
            token: "test-session-secret".into(),
            pid: std::process::id(),
            application: "test-only".into(),
        })
        .unwrap(),
    )
    .unwrap();
    let worker = std::thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let mut line = String::new();
        BufReader::new(stream.try_clone().unwrap())
            .read_line(&mut line)
            .unwrap();
        let request: AuthenticatedRequest = serde_json::from_str(&line).unwrap();
        assert_eq!(request.token, "test-session-secret");
        assert!(matches!(request.request.command, Command::Snapshot));
        let wrong = json!({"version":VERSION,"id":request.request.id + 1,"status":"ok","result":{"unexpected":true}});
        writeln!(stream, "{wrong}").unwrap();
    });
    let result = application::call(
        directory.path(),
        &application::Options {
            app_session: Some(path),
        },
        Command::Snapshot,
    )
    .await;
    worker.join().unwrap();
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("version/id mismatch"));
}

#[test]
fn cli_rejects_nonfinite_ranges_and_unknown_semantic_keys() {
    for arguments in [
        vec!["jet-bridge", "ui", "set-value", "range", "NaN"],
        vec!["jet-bridge", "ui", "set-value", "range", "inf"],
        vec!["jet-bridge", "ui", "key", "run_shell"],
    ] {
        assert!(cli::command().try_get_matches_from(arguments).is_err());
    }
    assert!(cli::command()
        .try_get_matches_from([
            "jet-bridge",
            "--app-session",
            "/tmp/test-session.json",
            "ui",
            "fill",
            "name",
            "中文🦀"
        ])
        .is_ok());
}
