use libeleven::rest::server::api_builder;
use serde_json::value::Value;

#[test]
fn test_versions_is_object_with_versions() {
    let api = api_builder();
    let app = app!(api);
    let res = call_app!(app, Get, "http://127.0.0.1:3000/_matrix/client/versions").ok().unwrap();
    let raw_body = resp_body!(res);
    let body: Value = raw_body.parse().unwrap();
    assert!(body.is_object());
    assert!(body.find("versions").is_some());

}

#[test]
fn test_supported_versions() {
    let api = api_builder();
    let app = app!(api);
    let res = call_app!(app, Get, "http://127.0.0.1:3000/_matrix/client/versions").ok().unwrap();
    let raw_body = resp_body!(res);
    let object: Value = raw_body.parse().unwrap();
    let versions_value: Vec<Value> = object.find("versions").unwrap().as_array().unwrap().clone();
    let mut versions: Vec<String> = Vec::new();
    for v in versions_value {
        versions.push(v.as_str().unwrap().to_string());
    }
    assert_eq!(versions, vec!["r0.2.0"]);
}
