use crate::serialization::Serialize;
use anyhow::{bail, ensure, Context, Result};
use rusqlite::OptionalExtension;
use std::io::Cursor;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPos3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl BlockPos3 {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    fn to_index(self) -> i64 {
        self.z * 0x1000000 + self.y * 0x1000 + self.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NodePos3 {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl NodePos3 {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn to_index(self) -> usize {
        assert!(self.x < Block::SIZE && self.y < Block::SIZE && self.z < Block::SIZE);

        self.z as usize * Block::SIZE * Block::SIZE
            + self.y as usize * Block::SIZE
            + self.x as usize
    }
}

pub struct Node {
    pub id: u16,
    pub param1: u8,
    pub param2: u8,
}

pub struct Block {
    node_data: Vec<u8>,
}

impl Block {
    pub const SIZE: usize = 16;

    pub fn deserialize(data: &[u8]) -> Result<Self> {
        let mut c = Cursor::new(data);
        let version = u8::deserialize(&mut c)?;

        ensure!(version == 28);

        todo!();
    }

    pub fn get_node(&self, pos: NodePos3) -> Node {
        let index = pos.to_index();

        todo!();
    }
}

pub trait Backend {
    fn get_block_data(&mut self, pos: BlockPos3) -> Result<Option<Vec<u8>>>;
    fn set_block_data(&mut self, pos: BlockPos3, data: &[u8]) -> Result<()>;
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

impl Backend for SqliteBackend {
    fn get_block_data(&mut self, pos: BlockPos3) -> Result<Option<Vec<u8>>> {
        let index = pos.to_index();
        let mut stmt = self
            .conn
            .prepare_cached("SELECT data FROM blocks WHERE pos = ?")
            .context("unable to prepare SQL statement")?;

        Ok(stmt.query_row([index], |row| row.get(0)).optional()?)
    }

    fn set_block_data(&mut self, pos: BlockPos3, data: &[u8]) -> Result<()> {
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

    pub fn get_block(&mut self, pos: BlockPos3) -> Result<Option<Block>> {
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
