use chrono::{Datelike, Utc};
use diesel::{Connection, PgConnection};
use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use tera::{Context, Tera};

use crate::{
    auth::hash_password,
    mail::HtmlMailer,
    models::{NewUser, RoleCode},
    repositories::{CrateRepo, RoleRepo, UserRepo},
};

fn load_db_connection() -> PgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("Error loading DB url from env");
    PgConnection::establish(&database_url).expect("Cannot connect to Postgres DB")
}

fn load_template_engine() -> Tera {
    Tera::new("templates/**/*.html").unwrap_or_else(|e| {
        panic!("Parsing error(s): {}", e);
    })
}

pub fn create_user(username: String, password: String, roles_code: Vec<String>) {
    let mut c = load_db_connection();
    let password_hash = hash_password(&password).unwrap();
    let new_user = NewUser {
        username: username,
        password: password_hash,
    };

    let role_code = roles_code
        .iter()
        .map(|r| RoleCode::from_string(r.to_string()).unwrap())
        .collect();
    log::info!("Roles {:?}", role_code);
    let user = UserRepo::create(&mut c, new_user, role_code).unwrap();
    println!("User created {:?}", user);
    let roles = RoleRepo::find_by_user(&mut c, &user).unwrap();
    println!("Roles assigned {:?}", roles);
}

pub fn list_users() {
    let mut c = load_db_connection();

    let users = UserRepo::find_with_roles(&mut c).unwrap();

    for user in users {
        println!("{:?}", user);
    }
}

pub fn delete_user(id: i32) {
    let mut c = load_db_connection();

    UserRepo::delete(&mut c, id).unwrap();
}

pub fn send_digest(to: String, hours_since: i32) {
    let mut c = load_db_connection();

    let crates = CrateRepo::find_since(&mut c, hours_since).unwrap();

    if crates.len() > 0 {
        println!("Sending digest for {} crates", crates.len());

        let tera = load_template_engine();

        let year = Utc::now().year();
        let mut context = Context::new();
        context.insert("crates", &crates);
        context.insert("year", &year);

        let smtp_host = std::env::var("SMTP_HOST").expect("Cannot load SMTP host from env");
        let smtp_username =
            std::env::var("SMTP_USERNAME").expect("Cannot load SMTP username from env");
        let smtp_password =
            std::env::var("SMTP_PASSWORD").expect("Cannot load SMTP password from env");

        let credentials = Credentials::new(smtp_username, smtp_password);

        let mailer = HtmlMailer {
            credentials: credentials,
            smtp_host: smtp_host,
            template_engine: tera,
        };
        mailer.send(to, "email/digest.html", &context).unwrap();
    }
}
