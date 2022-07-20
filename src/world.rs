use anyhow::{Context, Result};
use rusqlite::OptionalExtension;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pos3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

pub trait Backend {
    fn get_block_data(&mut self, pos: Pos3) -> Result<Option<Vec<u8>>>;
    fn set_block_data(&mut self, pos: Pos3, data: &[u8]) -> Result<()>;
}

pub struct SqliteBackend {
    conn: rusqlite::Connection,
}

impl SqliteBackend {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let flags = rusqlite::OpenFlags::SQLITE_OPEN_READ_WRITE
            | rusqlite::OpenFlags::SQLITE_OPEN_PRIVATE_CACHE;
        let conn = rusqlite::Connection::open_with_flags(path, flags)
            .context("unable to open SQLite database")?;

        Ok(Self { conn })
    }
}

fn pos_to_index(pos: Pos3) -> i64 {
    pos.z as i64 * 0x1000000 + pos.y as i64 * 0x1000 + pos.x as i64
}

impl Backend for SqliteBackend {
    fn get_block_data(&mut self, pos: Pos3) -> Result<Option<Vec<u8>>> {
        let index = pos_to_index(pos);
        let mut stmt = self
            .conn
            .prepare_cached("SELECT data FROM blocks WHERE pos = ?")
            .context("unable to prepare SQL statement")?;

        Ok(stmt.query_row([index], |row| row.get(0)).optional()?)
    }

    fn set_block_data(&mut self, pos: Pos3, data: &[u8]) -> Result<()> {
        todo!()
    }
}

pub struct World {
    backend: Box<dyn Backend>,
}

impl World {
    pub fn open<P: AsRef<Path>>(p: P) -> Result<Self> {
        todo!()
    }
}
