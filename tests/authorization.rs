use std::process::Command;

use common::{create_client, AUTH_BASE_URL};
use reqwest::StatusCode;
use rocket::serde::json::{json, Value};

pub mod common;

#[test]
fn test_login() {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("12345")
        .arg("admin")
        .output();

    let client = create_client();

    let response = client
        .post(AUTH_BASE_URL)
        .json(&json!({ "username": "test_admin", "password": "12345" }))
        .send()
        .expect("Failed to Login User");

    assert_eq!(response.status(), StatusCode::OK, "Failed to Login User");
    let json: Value = response.json().unwrap();
    println!("{:?}", json);
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);

    let response = client
        .post(AUTH_BASE_URL)
        .json(&json!({ "username": "test_admins", "password": "12345" }))
        .send()
        .expect("Failed to Login User");

    assert_eq!(
        response.status(),
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to Login User"
    );
}
