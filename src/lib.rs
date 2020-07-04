use diesel::{PgConnection, Connection};
use dotenv;
use std::env;

pub struct Jirachi<'a> {
    conn: PgConnection
}

impl Jirachi {
    pub fn new() -> Self {
        dotenv::dotenv.ok();
        let conn = PgConnection::establish(env::var("JIRACHI_DB_URL").unwrap().as_str());

        return Self {
            conn: conn.unwrap()
        }
    }


}


