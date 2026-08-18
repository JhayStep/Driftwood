use anyhow::{anyhow, Result};
use sled::Db;
use std::collections::BTreeMap;

const VERSION_TREE: &str = "__driftwood_versions";

pub struct Storage {
    pub(crate) db: Db,
}

impl Storage {
    pub fn open(path: &str) -> Result<Self> {
        Ok(Self {
            db: sled::open(path)?,
        })
    }

    pub fn get_doc(&self, id: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(id)?.map(|value| value.to_vec()))
    }

    pub fn put_doc(&self, id: &str, bytes: &[u8]) -> Result<()> {
        let versions = self.db.open_tree(VERSION_TREE)?;

        let current_version = match versions.get(id)? {
            Some(bytes) => decode_version(&bytes)?,
            None => 0,
        };

        let next_version = current_version + 1;

        self.db.insert(id, bytes)?;
        versions.insert(id, &next_version.to_be_bytes())?;

        self.db.flush()?;
        versions.flush()?;

        Ok(())
    }

    pub fn get_version(&self, id: &str) -> Result<u64> {
        let versions = self.db.open_tree(VERSION_TREE)?;

        match versions.get(id)? {
            Some(bytes) => decode_version(&bytes),
            None => Ok(0),
        }
    }

    pub fn digest(&self) -> Result<BTreeMap<String, u64>> {
        let versions = self.db.open_tree(VERSION_TREE)?;
        let mut digest = BTreeMap::new();

        for entry in &versions {
            let (key, value) = entry?;

            let doc_id = String::from_utf8(key.to_vec())?;
            let version = decode_version(&value)?;

            digest.insert(doc_id, version);
        }

        Ok(digest)
    }
}

fn decode_version(bytes: &[u8]) -> Result<u64> {
    let version_bytes: [u8; 8] = bytes
        .try_into()
        .map_err(|_| anyhow!("invalid stored document version"))?;

    Ok(u64::from_be_bytes(version_bytes))
}
