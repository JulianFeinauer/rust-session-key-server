#[macro_use]
extern crate diesel;

// mod super::models;
// mod schema;
mod handler;
mod repository;

use diesel_demo;

use actix_web::{HttpServer, App, middleware, web, HttpRequest, Responder};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use log::info;
use crate::handler::keys;

type Result = std::io::Result<()>;
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> Result {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv::dotenv().ok();

    let pool = init_connection();

    let bind = "127.0.0.1:8080";
    info!("Starting server at: {}", &bind);

    HttpServer::new(
        move || {
            App::new()
                .data(pool.clone())
                .wrap(middleware::Logger::default())
                .route("/", web::get().to(index))
                .route("/keys", web::get().to(keys::get_all))
                .route("/key/{id}", web::get().to(keys::get_by_public_id))
                // .route("/user", web::post().to(user::create))
                // .route("/user/{id}", web::put().to(user::update))
                // .route("/user/{id}", web::delete().to(user::delete))
        })
        .bind(bind)?
        .run().await
}

pub async fn index(_reg: HttpRequest) -> impl Responder {
    format!("welcome, index here")
}

fn init_connection() -> DbPool {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    r2d2::Pool::builder().build(manager).expect("Failed to create pool.")
}
