#![allow(non_local_definitions)] // Hash32 is outdated, need to fix fchashmap too

use serde::{Deserialize, Serialize};

#[derive(
    Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default,
)]
pub struct NodeId(pub(crate) usize);

impl NodeId {
    #[inline]
    pub fn from_index(idx: usize) -> Self {
        Self(idx)
    }

    #[inline]
    pub fn as_index(self) -> usize {
        self.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Node(pub(crate) String);

impl Node {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn name(&self) -> &str {
        &self.0
    }
}
