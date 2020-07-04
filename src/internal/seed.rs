use crate::schema::seeds;

#[derive(Debug, Queryable, Insertable, AsChangeset)]
pub struct Seed {
    pub prefix: String,
    pub index: i32
}