# jirachi
<a href="https://crates.io/crates/jirachi"><img src=https://img.shields.io/badge/crates.io-v0.1.6-orange></a>
<a href="https://docs.rs/jirachi/0.1.6"><img src=https://img.shields.io/badge/docs.rs-jirachi-blue></a>

<img src="https://i.ibb.co/2M8nC9V/rusty-jirachi-2.png" alt="rusty-jirachi-2" border="0">

A collision-resistant runtime agnostic key generator written in rust

## Steps to use:
- Install `jirachi_cli` by using `cargo install jirachi_cli`
- Set `JIRACHI_DB_URL` in your environment variables or in a `.env` file
- Run `jirachi setup` to initialize your database (The database must be created for this to work)

## Features:
- Collision resistance: This feature is optional and can be enabled in Cargo.toml by using <br/>
`jirachi: {version: "x.y.z", features=["collision-resistance"]}`
- Adjective noun key generator: This is a default feature which can generate adjective-noun pairs from an embedded dataset.
