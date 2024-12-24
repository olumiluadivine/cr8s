use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

use crate::common::RUSTACEAN_BASE_URL;

pub fn create_rustacean(client: &Client, name: &str, email: &str) -> Value {
    let response = client
        .post(RUSTACEAN_BASE_URL)
        .json(&json!({ "name": name, "email": email }))
        .send()
        .expect("Failed to create rustacean");

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "Rustacean creation failed"
    );
    response.json().expect("Failed to parse JSON response")
}

pub fn validate_rustacean(json: &Value, id: &Value, name: &str, email: &str) {
    assert_eq!(json["id"], *id, "ID does not match");
    assert_eq!(json["name"], name, "Name does not match");
    assert_eq!(json["email"], email, "Email does not match");
}