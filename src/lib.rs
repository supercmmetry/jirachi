

use diesel::{PgConnection, Connection, RunQueryDsl};
use diesel::prelude::*;
use dotenv;
use std::env;
use crate::schema::seeds::dsl::*;
use crate::internal::seed::Seed;

mod internal;
pub mod schema;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests;

pub struct Jirachi {
    conn: PgConnection,
    prefixes: Vec<String>,
    current_index: i32
}

impl Jirachi {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let conn = PgConnection::establish(env::var("JIRACHI_DB_URL").unwrap().as_str());

        return Self {
            conn: conn.unwrap(),
            prefixes: vec![],
            current_index: 0
        }
    }

    fn get_next_prefix(&mut self) -> String {
        let selected_prefix = self.prefixes[self.current_index as usize].clone();

        if self.current_index + 1  == self.prefixes.len() as i32 {
            self.current_index = 0;
        } else {
            self.current_index += 1;
        }

        selected_prefix
    }

    fn count(&self, other_prefix: String) -> i32 {
        let seed = seeds.find(other_prefix).first::<Seed>(&self.conn);
        return seed.unwrap().index
    }

    fn update(&self, seed: Seed) {
        diesel::update(seeds.filter(prefix.eq(seed.prefix.clone())))
            .set(index.eq(seed.index.clone()))
            .execute(&self.conn);
    }

    fn softload_prefixes(&mut self) {
        if self.prefixes.len() == 0 {
            let result = seeds.load::<Seed>(&self.conn);

            println!("{:#?}", result.unwrap());

            // for seed in &result.unwrap() {
            //     self.prefixes.push(seed.prefix.clone());
            // }
        }
    }

    pub fn wish(&mut self) -> String {
        self.softload_prefixes();
        let new_prefix = self.get_next_prefix();
        let count = self.count(new_prefix.clone());
        self.update(Seed {
            prefix: new_prefix.clone(),
            index: count.clone()
        });

        return new_prefix + count.to_string().as_str()
    }
}


