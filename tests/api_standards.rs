use libeleven::rest::server::api_builder;
use serde_json::value::Value;

#[test]
fn test_versions_is_array() {
    let api = api_builder();
    let app = app!(api);
    let res = call_app!(app, Get, "http://127.0.0.1:3000/_matrix/client/versions").ok().unwrap();
    let raw_body = resp_body!(res);
    let body: Value = raw_body.parse().unwrap();
    assert!(body.is_array());
}

#[test]
fn test_supported_versions() {
    let api = api_builder();
    let app = app!(api);
    let res = call_app!(app, Get, "http://127.0.0.1:3000/_matrix/client/versions").ok().unwrap();
    let raw_body = resp_body!(res);
    let parsed: Value = raw_body.parse().unwrap();
    let mut partial_body: Vec<Value> = Vec::new();
    partial_body.clone_from(parsed.as_array().unwrap());
    let mut body: Vec<String> = Vec::new();
    for v in partial_body {
        body.push(v.as_str().unwrap().to_string());
    }
    assert_eq!(body, vec!["r0.2.0"]);
}
