use anyhow::{bail, Context, Result};
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

pub struct Block {}

impl Block {
    pub fn deserialize(data: &[u8]) -> Result<Self> {
        todo!()
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

pub struct WorldMeta {
    backend: String,
    game: String,
}

impl WorldMeta {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let data = std::fs::read_to_string(path).context("unable to read world meta file")?;

        let mut backend = None;
        let mut game = None;

        for line in data.lines().map(|line| line.trim()) {
            if line.starts_with('#') {
                continue;
            }

            let (key, value) = line
                .split_once('=')
                .map(|(k, v)| (k.trim(), v.trim()))
                .context("invalid line")?;

            match key {
                "backend" => backend = Some(value.to_string()),
                "gameid" => game = Some(value.to_string()),
                _ => continue,
            }
        }

        Ok(Self {
            backend: backend.context("world.mt doesn't specify backend")?,
            game: game.context("world.mt doesn't specify game")?,
        })
    }
}

pub struct World {
    backend: Box<dyn Backend>,
}

impl World {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let meta = WorldMeta::from_file(path.as_ref().join("world.mt"))
            .context("unable to extract meta from world")?;

        println!("{}", meta.backend);

        let backend = match meta.backend.as_str() {
            "sqlite3" => {
                let path = path.as_ref().join("map.sqlite");
                Box::new(SqliteBackend::new(path)?)
            }
            _ => bail!("unknown world backend: {}", meta.backend),
        };

        Ok(Self { backend })
    }

    pub fn get_block(&mut self, pos: Pos3) -> Result<Option<Block>> {
        let data = self
            .backend
            .get_block_data(pos)
            .context("unable to retrieve block data")?;

        // Return None when backend returns no data
        let data = match data {
            Some(data) => data,
            None => return Ok(None),
        };

        let block = Block::deserialize(&data)?;

        Ok(Some(block))
    }
}
