use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{json, Value};

use crate::common::{rustacean::create_rustacean, CRATE_BASE_URL};

pub fn create_crate(
    client: &Client,
    name: &str,
    description: &str,
    code: &str,
    version: &str,
) -> Value {
    let rustaceans = create_rustacean(client, "Divine", "dbaby@dbaby.com");

    let response = client
        .post(CRATE_BASE_URL)
        .json(&json!({ "name": name, "description": description, "code": code, "version": version, "rustacean_id": rustaceans["id"] }))
        .send()
        .expect("Failed to create crate");

    assert_eq!(
        response.status(),
        StatusCode::CREATED,
        "Crate creation failed"
    );
    response.json().expect("Failed to parse JSON response")
}

pub fn validate_crate(
    json: &Value,
    id: &Value,
    name: &str,
    description: &str,
    code: &str,
    version: &str,
    rustaceans_id: &Value,
) {
    assert_eq!(json["id"], *id, "ID does not match");
    assert_eq!(json["name"], name, "Name does not match");
    assert_eq!(json["code"], code, "Code does not match");
    assert_eq!(
        json["description"], description,
        "Description does not match"
    );
    assert_eq!(json["version"], version, "Version does not match");
    assert_eq!(
        json["rustaceans_id"], *rustaceans_id,
        "Rustaceans_id does not match"
    );
}
