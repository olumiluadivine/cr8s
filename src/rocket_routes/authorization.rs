use crate::{
    auth::{authorize_user, Credentials},
    repositories::UserRepo,
    DbConn,
};
use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::{
    http::Status,
    response::status::Custom,
    serde::json::{serde_json::json, Json, Value}, time::PrimitiveDateTime,
};
use rocket_db_pools::{deadpool_redis::redis::AsyncCommands, Connection};
use serde::Serialize;

use super::CacheConn;

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    db: DbConn,
    mut cache: Connection<CacheConn>,
    credentials: Json<Credentials>,
) -> Result<Custom<Value>, Custom<Value>> {
    let username = credentials.username.clone();
    let user = db
        .run(move |c| {
            UserRepo::find_by_username(c, &username).map_err(|e| {
                eprintln!("Error: {}", e);
                Custom(Status::InternalServerError, json!("Something went wrong"))
            })
        })
        .await?;

    let session_id = authorize_user(&user, &credentials).map_err(|e| {
        eprintln!("Error: {}", e);
        Custom(Status::Unauthorized, json!(e))
    })?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map(|_| Custom(Status::Ok, json!(Auth::new(session_id))))
        .map_err(|e| {
            eprintln!("Error: {}", e);
            Custom(Status::InternalServerError, json!("Something went wrong"))
        })
}

#[derive(Serialize)]
struct Auth {
    token: String,
    response_at: DateTime<Utc>,
}

impl Auth {
    fn new(token: String) -> Self {
        Auth { token, response_at: Utc::now() }
    }
}