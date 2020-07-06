mod seed;
use crate::collision_resistant::seed::Seed;
use crate::schema::seeds::dsl::*;
use diesel::prelude::*;
use diesel::{Connection, PgConnection, RunQueryDsl};
use dotenv;
use std::env;

pub struct Jirachi {
    conn: PgConnection,
    prefixes: Vec<String>,
    current_index: i32,
}

impl Jirachi {
    pub fn new() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();
        let conn = PgConnection::establish(env::var("JIRACHI_DB_URL")?.as_str())?;

        return Ok(Self {
            conn,
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
        let seed = seeds.find(other_prefix).first::<Seed>(&self.conn);
        return Ok(seed?.index);
    }

    fn update(&self, seed: Seed) -> QueryResult<usize> {
        diesel::update(seeds.filter(prefix.eq(seed.prefix.clone())))
            .set(index.eq(seed.index.clone()))
            .execute(&self.conn)
    }

    fn softload_prefixes(&mut self) -> anyhow::Result<()> {
        if self.prefixes.len() == 0 {
            let result = seeds.load::<Seed>(&self.conn);

            for seed in &result? {
                self.prefixes.push(seed.prefix.clone());
            }
        }

        Ok(())
    }

    pub fn wish(&mut self) -> anyhow::Result<String> {
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

