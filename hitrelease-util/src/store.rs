use std::sync::Arc;

use dashmap::DashMap;
use uuid::Uuid;

use crate::Song;

/// Data structure to store song info for concurrent access.
///
/// Basically a thin wrapper around [`dashmap::DashMap`] with an opiniated
/// and purpose-built API.
pub struct Store {
    inner: Arc<DashMap<Uuid, Song>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(DashMap::new()),
        }
    }

    pub fn insert(&mut self, s: Song) {
        _ = self.inner.insert(s.id, s);
    }

    pub fn get(&self, id: &Uuid) -> Option<Song> {
        self.inner.get(id).map(|s| s.clone())
    }

    pub fn iter(&self) -> impl Iterator + use<'_> {
        self.inner.iter()
    }
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}
