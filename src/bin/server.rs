use rocket_db_pools::Database;

extern crate cr8s;

#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() {
    let _ = rocket::build()
        .mount(
            "/",
            routes![
                cr8s::rocket_routes::authorization::login,
                cr8s::rocket_routes::rustaceans::get_rustacean,
                cr8s::rocket_routes::rustaceans::get_rustaceans,
                cr8s::rocket_routes::rustaceans::create_rustacean,
                cr8s::rocket_routes::rustaceans::update_rustacean,
                cr8s::rocket_routes::rustaceans::delete_rustacean,
                cr8s::rocket_routes::crates::get_crates,
                cr8s::rocket_routes::crates::get_crate,
                cr8s::rocket_routes::crates::create_crate,
                cr8s::rocket_routes::crates::update_crate,
                cr8s::rocket_routes::crates::delete_crate,
            ],
        )
        .attach(cr8s::rocket_routes::DbConn::fairing())
        .attach(cr8s::rocket_routes::CacheConn::init())
        .launch()
        .await;
}
