#[cfg(feature = "collision-resistant")]
pub mod collision_resistant;
pub mod adjective_noun;

#[cfg(feature = "collision-resistant")]
mod schema;

#[cfg(feature = "collision-resistant")]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate anyhow;

pub trait Wishable {
    fn wish(&mut self) -> anyhow::Result<String>;
}

