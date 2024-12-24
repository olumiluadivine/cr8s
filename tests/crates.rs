use common::{
    crates::{create_crate, validate_crate},
    endpoint, get_valid_session_editor, CRATE_BASE_URL,
};
use reqwest::StatusCode;
use rocket::serde::json::{json, Value};

pub mod common;

#[test]
fn test_get_crates() {
    let client = get_valid_session_editor();

    let response = client
        .get(CRATE_BASE_URL)
        .send()
        .expect("Failed to send initial GET request for crates");
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET crates failed initially"
    );

    let crate1 = create_crate(&client, "Divine", "Divine Crate", "https", "0.1");
    let crate2 = create_crate(&client, "Favour", "Favour Crate", "https", "0.1");

    let response = client
        .get(CRATE_BASE_URL)
        .send()
        .expect("Failed to send GET request for crates after creation");
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET crates failed after creation"
    );

    let crates: Value = response
        .json()
        .expect("Failed to parse JSON response from GET crates");
    let crates_array = crates.as_array().expect("Expected an array of crates");

    assert!(
        crates_array.contains(&crate1),
        "Response does not contain crate1"
    );
    assert!(
        crates_array.contains(&crate2),
        "Response does not contain crate2"
    );
}

#[test]
fn test_create_and_get_crate() {
    let client = get_valid_session_editor();
    let created = create_crate(&client, "John Doe", "John Doe Crate", "https", "0.1");

    let response = client
        .get(endpoint(&created["id"], CRATE_BASE_URL))
        .send()
        .expect("Failed to send GET request for single crate");

    assert_eq!(response.status(), StatusCode::OK, "GET single crate failed");

    let retrieved: Value = response.json().expect("Failed to parse JSON response");
    validate_crate(
        &retrieved,
        &created["id"],
        "John Doe",
        "John Doe Crate",
        "https",
        "0.1",
        &created["rustaceans_id"],
    );
}

#[test]
fn test_update_crate() {
    let client = get_valid_session_editor();
    let created = create_crate(&client, "John Doe", "John Doe Crate", "https", "0.1");

    let response = client
        .put(endpoint(&created["id"], CRATE_BASE_URL))
        .json(&json!({ "name": "John Dove", "description": "John Dove Crate", "code": "http", "version": "0.2" }))
        .send()
        .expect("Failed to send PUT request to update crate");

    assert_eq!(response.status(), StatusCode::OK, "Update crate failed");

    let updated: Value = response.json().expect("Failed to parse JSON response");
    validate_crate(
        &updated,
        &created["id"],
        "John Dove",
        "John Dove Crate",
        "http",
        "0.2",
        &created["rustaceans_id"],
    );
}

#[test]
fn test_delete_crate() {
    let client = get_valid_session_editor();
    let created = create_crate(&client, "John Doe", "John Doe Crate", "https", "0.1");

    let response = client
        .delete(endpoint(&created["id"], CRATE_BASE_URL))
        .send()
        .expect("Failed to send DELETE request to delete crate");

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "Delete crate failed"
    );
}
