use rocket::{
    http::Status,
    response::status::{Custom, NoContent},
    serde::json::{serde_json::json, Json, Value},
};

use crate::{
    models::{Crate, NewCrate, User},
    repositories::CrateRepo,
    DbConn,
};

use super::EditorUser;

#[get("/crt")]
pub async fn get_crates(db: DbConn, _user: User) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        CrateRepo::find_multiple(c, 100)
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[get("/crt/<id>")]
pub async fn get_crate(db: DbConn, _user: User, id: i32) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepo::find(c, id)
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[post("/crt", format = "json", data = "<data>")]
pub async fn create_crate(
    db: DbConn,
    _user: EditorUser,
    data: Json<NewCrate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(|c| {
        CrateRepo::create(c, data.into_inner())
            .map(|result| Custom(Status::Created, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[put("/crt/<id>", format = "json", data = "<data>")]
pub async fn update_crate(
    db: DbConn,
    _user: EditorUser,
    id: i32,
    data: Json<Crate>,
) -> Result<Custom<Value>, Custom<Value>> {
    db.run(move |c| {
        CrateRepo::save(c, id, data.into_inner())
            .map(|result| Custom(Status::Ok, json!(result)))
            .map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
    })
    .await
}

#[delete("/crt/<id>")]
pub async fn delete_crate(
    db: DbConn,
    _user: EditorUser,
    id: i32,
) -> Result<NoContent, Custom<Value>> {
    db.run(move |c| {
        CrateRepo::delete(c, id).map(|_| NoContent).map_err(|e| {
            eprintln!("Error: {}", e);
            Custom(Status::InternalServerError, json!("Something went wrong"))
        })
    })
    .await
}
