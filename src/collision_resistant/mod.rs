mod seed;
use crate::collision_resistant::seed::Seed;
use crate::schema::seeds::dsl::*;
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, r2d2};
use dotenv;
use std::env;
use crate::Wishable;
use diesel::r2d2::ConnectionManager;
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Jirachi {
    pool: Pool,
    prefixes: Vec<String>,
    current_index: i32,
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
        let pool = Pool::new(manager).expect("db pool");

        return Ok(Self {
            pool,
            prefixes: vec![],
            current_index: 0,
        });
    }

    fn get_next_prefix(&mut self) -> anyhow::Result<String> {
        if self.current_index >= self.prefixes.len() as i32 {
            return Err(anyhow!("Insufficient prefixes were loaded."));
        }

        let selected_prefix = self.prefixes[self.current_index as usize].clone();

        if self.current_index + 1 == self.prefixes.len() as i32 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        Ok(selected_prefix)
    }

    fn count(&self, other_prefix: String) -> anyhow::Result<i32> {
        let result = self.pool.get();
        if result.is_err() {
            return Err(anyhow!("Error occured while fetching a connection."))
        }

        let conn = DbConn(result.unwrap());

        let seed = seeds.find(other_prefix).first::<Seed>(conn.deref());
        return Ok(seed?.index);
    }

    fn update(&self, seed: Seed) -> QueryResult<usize> {
        let result = self.pool.get();
        if result.is_err() {
            return Err(diesel::result::Error::NotFound);
        }

        let conn = DbConn(result.unwrap());

        diesel::update(seeds.filter(prefix.eq(seed.prefix.clone())))
            .set(index.eq(seed.index.clone()))
            .execute(conn.deref())
    }

    fn softload_prefixes(&mut self) -> anyhow::Result<()> {
        let result = self.pool.get();
        if result.is_err() {
            return Err(anyhow!("Error occured while fetching a connection."))
        }

        let conn = DbConn(result.unwrap());

        if self.prefixes.len() == 0 {
            let result = seeds.load::<Seed>(conn.deref());

            for seed in &result? {
                self.prefixes.push(seed.prefix.clone());
            }
        }

        Ok(())
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
        self.softload_prefixes()?;
        let new_prefix = self.get_next_prefix()?;
        let count = self.count(new_prefix.clone())?;
        self.update(Seed {
            prefix: new_prefix.clone(),
            index: count.clone() + 1,
        })?;

        return Ok(new_prefix + count.to_string().as_str());
    }
}

