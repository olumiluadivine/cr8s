pub mod crates;
pub mod rustacean;

use std::process::Command;

use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
    StatusCode,
};
use rocket::serde::json::{json, Value};

pub const RUSTACEAN_BASE_URL: &str = "http://127.0.0.1:8000/rts";
pub const CRATE_BASE_URL: &str = "http://127.0.0.1:8000/crt";
pub const AUTH_BASE_URL: &str = "http://127.0.0.1:8000/login";

pub fn endpoint(path: &Value, base_url: &str) -> String {
    format!("{}/{}", base_url, path)
}

pub fn create_client() -> Client {
    Client::new()
}

fn get_login_for_user(username: &str, password: &str, roles: &str) -> String {
    let _ = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg(username)
        .arg(password)
        .arg(roles)
        .output();

    let client = create_client();

    let response = client
        .post(AUTH_BASE_URL)
        .json(&json!({ "username": username, "password": password }))
        .send()
        .expect("Failed to Login User");

    assert_eq!(response.status(), StatusCode::OK, "Failed to Login User");
    let json: Value = response.json().unwrap();
    println!("{:?}", json);
    assert!(json.get("token").is_some());
    assert_eq!(json["token"].as_str().unwrap().len(), 128);
    format!("Bearer {}", json["token"].as_str().unwrap())
}

pub fn get_valid_session_admin() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&get_login_for_user("admin", "12345", "admin")).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_valid_session_editor() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&get_login_for_user("editor", "12345", "editor")).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub fn get_valid_session_viewer() -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&get_login_for_user("viewer", "12345", "viewer")).unwrap(),
    );
    ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
}
