//! domain/id.rs
use serde::{Deserialize, Serialize};
use std::{ops::Deref, str::FromStr};
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TodoId(Uuid);

impl Default for TodoId {
    fn default() -> Self {
        Self::new()
    }
}

impl TodoId {
    /// Generate a fresh identifier.
    pub fn new() -> Self {
        TodoId(Uuid::new_v4())
    }
}

impl Deref for TodoId {
    type Target = Uuid;
    fn deref(&self) -> &Uuid {
        &self.0
    }
}

impl FromStr for TodoId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(TodoId).map_err(|_| IdError::Invalid)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum IdError {
    #[error("malformed UUID")]
    Invalid,
}
