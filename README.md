# jirachi
A collision-resistant runtime agnostic key generator written in rust

## Steps to use:
- Install `jirachi_cli` by using `cargo install jirachi_cli`
- Set `JIRACHI_DB_URL` in your environment variables or in a `.env` file
- Run `jirachi setup` to initialize your database (The database must be created for this to work)

## Features:
- Collision resistance: This feature is optional and can be enabled in Cargo.toml by using <br/>
`jirachi: {version: "x.y.z", features=["collision-resistance"]}`
- Adjective noun key generator: This is a default feature which can generate adjective-noun pairs from an embedded dataset.
