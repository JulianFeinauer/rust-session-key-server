extern crate diesel;

use actix_web::{App, HttpRequest, HttpServer, middleware, Responder, web};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;

use crate::handler::keys;

mod handler;
mod repository;

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

fn add_padding(input: &mut Vec<u8>) {
    // Input has to be 16 byte aligned!
    let remainder = 16 - input.len() % 16;
    for _ in 0..remainder {
        input.push(0);
    }
}

fn remove_padding(input: &mut Vec<u8>) {
    let mut unpadded_plaintext = vec![0u8; 0];

    for sign in input.into_iter() {
        let sign: u8 = *sign;
        if sign > 0 {
            unpadded_plaintext.push(sign)
        }
    }

    input.clear();
    input.append(&mut unpadded_plaintext);
}

#[cfg(test)]
mod tests {
    use std::str;

    use crypto::aessafe;
    use crypto::symmetriccipher::{BlockDecryptor, BlockEncryptor};
    use crate::{add_padding, remove_padding};

    extern crate base64;

    #[test]
    fn encrypt_decrypt() {
        let key = "hWmZq3t6w9z$C&F)".as_bytes();
        let encryptor = aessafe::AesSafe128Encryptor::new(key);

        let mut uncleaned_input = "Hallo Julian!!".as_bytes().to_vec();

        add_padding(&mut uncleaned_input);

        let input = uncleaned_input.as_slice();

        println!("Cleaned Input: '{}'", str::from_utf8(&input).expect(""));

        let mut output = vec![0u8; input.len()];

        println!("Input: {}, Output: {}, Input * 4: {}", input.len(), output.len(), input.len() * 4);

        encryptor.encrypt_block(input, &mut output);

        for val in &output {
            println!("Output: {}", val);
        }

        println!("Base64: {}", base64::encode(&output));

        let decryptor = aessafe::AesSafe128Decryptor::new(key);

        let mut plaintext = vec![0u8; output.len()];
        decryptor.decrypt_block(&output, &mut plaintext);

        remove_padding(&mut plaintext);

        let plaintext_as_str = str::from_utf8(&plaintext.as_slice()).expect("Waah");
        println!("{}", plaintext_as_str);

        assert_eq!(plaintext_as_str, "Hallo Julian!!")
    }
}
