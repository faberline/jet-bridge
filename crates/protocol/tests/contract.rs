use jet_bridge_protocol::{Command, NativeSession, Outcome, Request, Response};
use serde_json::json;

#[test]
fn external_wire_json_preserves_unicode_and_flat_response_outcomes() {
    let request: Request = serde_json::from_value(json!({
        "version":1,"id":7,"command":{"method":"fill","id":"name","value":"中🦀"}
    }))
    .unwrap();
    assert!(matches!(request.command, Command::Fill { id, value } if id=="name" && value=="中🦀"));
    let raw = json!({"version":1,"id":7,"status":"ok","result":{"value":"中🦀"}});
    let response: Response = serde_json::from_value(raw.clone()).unwrap();
    assert!(matches!(&response.outcome, Outcome::Ok { result } if result["value"]=="中🦀"));
    assert_eq!(serde_json::to_value(response).unwrap(), raw);
}

#[test]
fn malformed_commands_fail_instead_of_choosing_another_operation() {
    for raw in [
        json!({"method":"run_shell","command":"anything"}),
        json!({"method":"fill","id":"name","value":17}),
        json!({"method":"activate","id":"button","unknown":true}),
    ] {
        assert!(serde_json::from_value::<Command>(raw).is_err());
    }
}

#[test]
fn native_session_debug_output_does_not_reveal_its_token() {
    let session = NativeSession {
        version: 1,
        endpoint: "127.0.0.1:1234".into(),
        token: "private-test-token".into(),
        pid: 123,
        application: "Fixture".into(),
    };
    let debug = format!("{session:?}");
    assert!(!debug.contains("private-test-token"));
    assert!(debug.contains("[redacted]"));
}
