use diesel::{OptionalExtension, PgConnection, QueryDsl, RunQueryDsl};
use diesel_demo::models::SessionKeys;
use diesel_demo::schema::session_keys::dsl::*;

pub fn find_all(conn: &PgConnection) -> Result<Vec<SessionKeys>, diesel::result::Error> {
    let result = session_keys.load::<SessionKeys>(conn)?;
    Ok(result)
}

pub fn find_by_public_id(conn: &PgConnection, user_id: uuid::Uuid) -> Result<Option<SessionKeys>, diesel::result::Error> {
    let result = session_keys.find(user_id).first::<SessionKeys>(conn).optional()?;
    Ok(result)
}
