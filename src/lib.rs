#[cfg(feature = "collision-resistant")]
pub mod collision_resistant;

#[cfg(feature = "collision-resistant")]
mod schema;

#[cfg(feature = "collision-resistant")]
#[macro_use]
extern crate diesel;

#[macro_use]
extern crate anyhow;
