// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::fmt;

use nostr::{key, secp256k1};
use nostr_database::DatabaseError;
use rusqlite::types::FromSqlError;
use tokio::task::JoinError;

/// Store error
#[derive(Debug)]
pub enum Error {
    /// Sqlite error
    Sqlite(rusqlite::Error),
    /// Pool error
    Thread(JoinError),
    /// From SQL error
    FromSql(FromSqlError),
    /// Url error
    Url(nostr::types::url::ParseError),
    /// Pool error
    Key(key::Error),
    /// Pool error
    Secp256k1(secp256k1::Error),
    /// Migration error
    NewerDbVersion { current: usize, other: usize },
    /// Not found
    NotFound(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sqlite(e) => write!(f, "{e}"),
            Self::Thread(e) => write!(f, "{e}"),
            Self::FromSql(e) => write!(f, "{e}"),
            Self::Url(e) => write!(f, "{e}"),
            Self::Key(e) => write!(f, "{e}"),
            Self::Secp256k1(e) => write!(f, "{e}"),
            Self::NewerDbVersion { current, other } => write!(f, "Database version is newer than supported by this executable (v{current} > v{other})"),
            Self::NotFound(item) => write!(f, "sqlite: {} not found", item),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Self::Sqlite(err)
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Self {
        Self::Thread(err)
    }
}

impl From<FromSqlError> for Error {
    fn from(err: FromSqlError) -> Self {
        Self::FromSql(err)
    }
}

impl From<nostr::types::url::ParseError> for Error {
    fn from(err: nostr::types::url::ParseError) -> Self {
        Self::Url(err)
    }
}

impl From<key::Error> for Error {
    fn from(err: key::Error) -> Self {
        Self::Key(err)
    }
}

impl From<secp256k1::Error> for Error {
    fn from(err: secp256k1::Error) -> Self {
        Self::Secp256k1(err)
    }
}

impl From<Error> for DatabaseError {
    fn from(e: Error) -> Self {
        Self::backend(e)
    }
}
