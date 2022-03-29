use std::fmt;
use std::fmt::Formatter;
use std::path::Display;
use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use super::schema::session_keys;

#[derive(Serialize,Insertable,Queryable)]
#[table_name="session_keys"]
pub struct SessionKeys {
    pub id: uuid::Uuid,
    pub session_key: String
}

impl fmt::Display for SessionKeys {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.id, self.session_key)
    }
}
