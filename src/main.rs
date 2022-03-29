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

#[cfg(test)]
mod tests {
    use std::ops::Deref;
    use std::slice;
    use std::str;
    use crypto::aessafe;
    use crypto::symmetriccipher::{BlockDecryptor, BlockEncryptor};
    extern crate base64;

    #[test]
    fn encrypt_decrypt() {
        let key = "hWmZq3t6w9z$C&F)".as_bytes();
        let encryptor = aessafe::AesSafe128Encryptor::new(key);

        let mut uncleaned_input = "Hallo Julian!!".as_bytes().to_vec();


        // Input has to be 16 byte aligned?
        let remainder = 16 - uncleaned_input.len() % 16;

        println!("Remainder {}", remainder);

        for _ in 0..remainder {
            // uncleaned_input.push(" ".as_bytes()[0])
            uncleaned_input.push(0);
        }

        let input = uncleaned_input.as_slice();

        println!("Cleaned Input: '{}'", str::from_utf8(&input).expect(""));

        let mut output = vec![0u8; input.len()];

        println!("Input: {}, Output: {}, Input * 4: {}", input.len(), output.len(), input.len()*4);

        encryptor.encrypt_block(input, &mut output);

        for val in &output {
            println!("Output: {}", val);
        }

        println!("Base64: {}", base64::encode(&output));

        let decryptor = aessafe::AesSafe128Decryptor::new(key);

        let mut plaintext = vec![0u8; output.len()];
        decryptor.decrypt_block(&output, &mut plaintext);

        let plaintext_as_str = str::from_utf8(&plaintext).expect("Waah");
        println!("{}", plaintext_as_str)
    }
}
