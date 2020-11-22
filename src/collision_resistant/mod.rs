mod seed;

use crate::collision_resistant::seed::Seed;
use crate::schema::seeds::dsl::*;
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, r2d2};
use dotenv;
use std::env;
use crate::Wishable;
use diesel::r2d2::ConnectionManager;
use diesel::dsl::min;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Jirachi {
    pool: Pool
}

impl Jirachi {
    /// Returns a anyhow::Result containing a Jirachi instance
    ///
    /// # Example
    ///
    /// ```
    /// use jirachi::collision_resistant::Jirachi;
    ///
    /// let jirachi = Jirachi::new().unwrap();
    /// ```
    pub fn new() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let manager = ConnectionManager::<PgConnection>::new(env::var("JIRACHI_DB_URL")?);
        let pool_size = env::var("JIRACHI_POOL_SIZE")?.parse::<u32>().expect("pool size");
        let pool = Pool::builder().max_size(pool_size).build(manager).expect("db pool");

        return Ok(Self {
            pool
        });
    }
}

impl Wishable for Jirachi {
    /// Returns a key created using random adjective-noun pairs
    ///
    /// # Example
    ///
    /// ```
    /// use jirachi::collision_resistant::Jirachi;
    /// use jirachi::Wishable;
    ///
    /// let mut jirachi = Jirachi::new().unwrap();
    /// let wish = jirachi.wish().unwrap();
    /// ```
    fn wish(&mut self) -> anyhow::Result<String> {
        let conn = self.pool.get()?;

        conn.transaction::<_, diesel::result::Error, _>(|| {
            let seed_index_option: Option<i32> = seeds.select(min(index)).first(&conn)?;

            if seed_index_option.is_none() {
                return Err(diesel::result::Error::RollbackTransaction);
            }

            let seed_index = seed_index_option.unwrap();

            let next_prefix: String = seeds.filter(index.eq(seed_index.clone()))
                .select(prefix)
                .first(&conn)?;

            diesel::update(seeds.filter(prefix.eq(next_prefix.clone())))
                .set(index.eq(seed_index.clone() + 1))
                .execute(&conn);

            Ok(Ok(next_prefix + seed_index.to_string().as_str()))
        })?
    }
}

