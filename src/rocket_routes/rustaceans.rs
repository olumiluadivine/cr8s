use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{NewRustacean, Rustacean, User},
    repositories::RustaceanRepo,
    DbConn,
};

use super::EditorUser;

#[get("/rts")]
pub async fn get_rustaceans(db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        RustaceanRepo::find_multiple(c, 100)
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[get("/rts/<id>")]
pub async fn get_rustacean(
    db: DbConn,
    _user: User,
    id: i32,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepo::find(c, id)
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[post("/rts", format = "json", data = "<data>")]
pub async fn create_rustacean(
    db: DbConn,
    _user: EditorUser,
    data: Json<NewRustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        RustaceanRepo::create(c, data.into_inner())
            .map(|result| Custom(Status::Created, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[put("/rts/<id>", format = "json", data = "<data>")]
pub async fn update_rustacean(
    db: DbConn,
    _user: EditorUser,
    id: i32,
    data: Json<Rustacean>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepo::save(c, id, data.into_inner())
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[delete("/rts/<id>")]
pub async fn delete_rustacean(
    db: DbConn,
    _user: EditorUser,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        RustaceanRepo::delete(c, id)
            .map(|_| NoContent)
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}
