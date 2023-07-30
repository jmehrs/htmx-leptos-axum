use rusqlite::Connection;
use rusqlite_migration::Migrations;
use include_dir::{Dir, include_dir};
use anyhow::Result;
use lazy_static::lazy_static;

static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> = Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

pub fn init_db() -> Result<Connection> {
    let mut conn = Connection::open("./app.db")?;

    // Update the database schema, atomically
    MIGRATIONS.to_latest(&mut conn)?;

    Ok(conn)
}