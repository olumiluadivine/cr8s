use common::{
    endpoint, get_valid_session_editor,
    rustacean::{create_rustacean, validate_rustacean},
    RUSTACEAN_BASE_URL,
};
use reqwest::StatusCode;
use rocket::serde::json::{json, Value};

pub mod common;

#[test]
fn test_get_rustaceans() {
    let client = get_valid_session_editor();

    let response = client
        .get(RUSTACEAN_BASE_URL)
        .send()
        .expect("Failed to send initial GET request for rustaceans");
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET rustaceans failed initially"
    );

    let rustacean1 = create_rustacean(&client, "Divine", "divine@dbaby.com");
    let rustacean2 = create_rustacean(&client, "Favour", "favour@dbaby.com");

    let response = client
        .get(RUSTACEAN_BASE_URL)
        .send()
        .expect("Failed to send GET request for rustaceans after creation");
    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET rustaceans failed after creation"
    );

    let rustaceans: Value = response
        .json()
        .expect("Failed to parse JSON response from GET rustaceans");
    let rustaceans_array = rustaceans
        .as_array()
        .expect("Expected an array of rustaceans");

    assert!(
        rustaceans_array.contains(&rustacean1),
        "Response does not contain rustacean1"
    );
    assert!(
        rustaceans_array.contains(&rustacean2),
        "Response does not contain rustacean2"
    );
}

#[test]
fn test_create_and_get_rustacean() {
    let client = get_valid_session_editor();
    let created = create_rustacean(&client, "John Doe", "jd@gmail.com");
    println!("{:?}", created);
    let response = client
        .get(endpoint(&created["id"], RUSTACEAN_BASE_URL))
        .send()
        .expect("Failed to send GET request for single rustacean");

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "GET single rustacean failed"
    );

    let retrieved: Value = response.json().expect("Failed to parse JSON response");
    validate_rustacean(&retrieved, &created["id"], "John Doe", "jd@gmail.com");
}

#[test]
fn test_update_rustacean() {
    let client = get_valid_session_editor();
    let created = create_rustacean(&client, "John Doe", "jd@gmail.com");

    let response = client
        .put(endpoint(&created["id"], RUSTACEAN_BASE_URL))
        .json(&json!({ "name": "John Dove", "email": "j0vd@gmail.com" }))
        .send()
        .expect("Failed to send PUT request to update rustacean");

    assert_eq!(response.status(), StatusCode::OK, "Update rustacean failed");

    let updated: Value = response.json().expect("Failed to parse JSON response");
    validate_rustacean(&updated, &created["id"], "John Dove", "j0vd@gmail.com");
}

#[test]
fn test_delete_rustacean() {
    let client = get_valid_session_editor();
    let created = create_rustacean(&client, "John Doe", "jd@gmail.com");

    let response = client
        .delete(endpoint(&created["id"], RUSTACEAN_BASE_URL))
        .send()
        .expect("Failed to send DELETE request to delete rustacean");

    assert_eq!(
        response.status(),
        StatusCode::NO_CONTENT,
        "Delete rustacean failed"
    );
}
