use diesel::PgConnection;
use rocket::request::FromRequest;
use rocket_db_pools::{
    deadpool_redis::{redis::AsyncCommands, Pool},
    Connection, Database,
};
use rocket_sync_db_pools::database;

use crate::{
    models::{RoleCode, User},
    repositories::{RoleRepo, UserRepo},
};

pub mod authorization;
pub mod crates;
pub mod rustaceans;

#[database("postgres")]
pub struct DbConn(PgConnection);

#[derive(Database)]
#[database("redis")]
pub struct CacheConn(Pool);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let db = request.guard::<DbConn>().await.unwrap();
        let mut cache = request.guard::<Connection<CacheConn>>().await.unwrap();

        let session_id = request
            .headers()
            .get_one("Authorization")
            .map(|data| data.split_whitespace().collect::<Vec<&str>>())
            .filter(|v| v.len() == 2 && v[0] == "Bearer");

        if let Some(session_id) = session_id {
            let user_id = cache
                .get::<_, i32>(format!("sessions/{}", session_id[1]))
                .await;

            if let Ok(user_id) = user_id {
                match db.run(move |c| UserRepo::find(c, user_id)).await {
                    Ok(user) => {
                        return rocket::request::Outcome::Success(user);
                    }
                    _ => {
                        return rocket::request::Outcome::Forward(
                            rocket::http::Status::Unauthorized,
                        );
                    }
                }
            }
        }

        rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized)
    }
}

pub struct EditorUser(User);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for EditorUser {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        let user = match request.guard::<User>().await {
            rocket::outcome::Outcome::Success(user) => user,
            _ => return rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized),
        };

        let db = match request.guard::<DbConn>().await {
            rocket::outcome::Outcome::Success(db) => db,
            _ => return rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized),
        };

        let user_clone = user.clone();
        let has_access = db
            .run(move |c| match RoleRepo::find_by_user(c, &user_clone) {
                Ok(roles) => {
                    log::info!("User roles {:?}", roles);
                    roles
                        .into_iter()
                        .any(|role| matches!(role.code, RoleCode::Editor | RoleCode::Admin))
                }
                Err(_) => false,
            })
            .await;

        if has_access {
            rocket::request::Outcome::Success(EditorUser(user))
        } else {
            rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized)
        }
    }
}
