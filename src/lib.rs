#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use uuid::Uuid;

use crate::models::SessionKeys;
use crate::schema::session_keys;

pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("No Database url set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_key(conn: &PgConnection, session: &str) -> SessionKeys {
    let new_key = SessionKeys {
            id: Uuid::new_v4(),
            session_key: session.to_string()
    };

        diesel::insert_into(session_keys::table)
            .values(&new_key)
            .get_result::<SessionKeys>(conn)
            .expect("Unable to save")
}

pub fn read_key(conn: &PgConnection, primary_key: &str) -> SessionKeys {
    use schema::session_keys::dsl::*;

    let key = Uuid::parse_str(primary_key).expect("Unable to parse");
    session_keys.find(key).get_result::<SessionKeys>(conn).expect("No id found")
}

#[cfg(test)]
mod tests {
    extern crate diesel;

    use diesel::PgConnection;
    use uuid::Uuid;

    use crate::{establish_connection, insert_key, read_key};
    use crate::models::SessionKeys;
    use crate::schema::session_keys::dsl::*;

    use self::diesel::prelude::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn connect() {
        use crate::schema::session_keys;
        let connection = establish_connection();

        // Insert something
        let inserted_key = insert_key(&connection, "Hallo");

        println!("Newly inserted id: {}", inserted_key.id);

        let read_key = read_key(&connection, inserted_key.id.to_string().as_str());

        println!("Read key: {}", read_key);

        let results = session_keys
            .limit(5)
            .load::<SessionKeys>(&connection)
            .expect("Error Loading Keys");

        for key in results {
            println!("{} -> {}", key.id, key.session_key);
        }
    }
}
