use std::io::Write;

use crate::schema::{crates, roles, rustaceans, users, users_roles};
use chrono::NaiveDateTime;
use diesel::{pg::Pg, sql_types::Text};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct Rustacean {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rustaceans)]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Rustacean))]
#[diesel(table_name = crates)]
pub struct Crate {
    #[serde(skip_deserializing)]
    id: i32,
    #[serde(skip_deserializing)]
    rustacean_id: i32,
    pub code: String,
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    #[serde(skip_deserializing)]
    created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crates)]
pub struct NewCrate {
    rustacean_id: i32,
    code: String,
    name: String,
    version: String,
    description: Option<String>,
}

#[derive(Identifiable, Queryable, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Identifiable, Queryable, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Identifiable, Associations, Queryable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    id: i32,
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(Debug, FromSqlRow, AsExpression)]
#[sql_type = "diesel::sql_types::Text"]
pub enum RoleCode {
    Admin,
    Editor,
    Viewer,
}

impl RoleCode {
    pub fn from_string(string: String) -> Result<Self, Box<dyn std::error::Error>> {
        match string.as_str() {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Err("Invalid role code".into()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            RoleCode::Admin => "admin",
            RoleCode::Editor => "editor",
            RoleCode::Viewer => "viewer",
        }
    }

    pub fn as_str_static(&self) -> &'static str {
        match self {
            RoleCode::Admin => "admin",
            RoleCode::Editor => "editor",
            RoleCode::Viewer => "viewer",
        }
    }
}

impl diesel::deserialize::FromSql<Text, Pg> for RoleCode {
    fn from_sql(
        bytes: <Pg as diesel::backend::Backend>::RawValue<'_>,
    ) -> diesel::deserialize::Result<Self> {
        let role_code = <String as diesel::deserialize::FromSql<Text, Pg>>::from_sql(bytes)?;
        match role_code.as_str() {
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl diesel::serialize::ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}
