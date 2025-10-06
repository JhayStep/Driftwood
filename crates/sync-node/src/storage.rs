use anyhow::Result;
use sled::Db;

pub struct Storage { pub(crate) db: Db }

impl Storage {
    pub fn open(path: &str) -> Result<Self> { Ok(Self { db: sled::open(path)? }) }

    pub fn get_doc(&self, id: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(id)?.map(|v| v.to_vec()))
    }

    pub fn put_doc(&self, id: &str, bytes: &[u8]) -> Result<()> { self.db.insert(id, bytes)?; self.db.flush()?; Ok(()) }
}
