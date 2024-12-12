// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::path::Path;
use std::sync::Arc;

use nostr_database::prelude::*;
use rusqlite::config::DbConfig;
use rusqlite::Connection;
use tokio::sync::RwLock;

mod error;
mod migration;
mod pool;

use self::error::Error;
use self::migration::STARTUP_SQL;
use self::pool::Pool;

#[derive(Debug, Clone)]
pub struct Store {
    pool: Pool,
}

impl Store {
    pub async fn open<P>(path: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        let conn = Connection::open(path)?;
        let pool: Pool = Pool::new(conn);

        // Execute migrations
        migration::run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn event_by_id(&self, id: &EventId) -> Result<Option<Event>, Error> {
        let event_id = id.to_bytes();
        self.pool
            .interact(move |conn| get_event_by_id(conn, event_id))
            .await?
    }

    pub async fn wipe(&self) -> Result<(), Error> {
        self.pool
            .interact(|conn| {
                // Reset DB
                conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, true)?;
                conn.execute("VACUUM;", [])?;
                conn.set_db_config(DbConfig::SQLITE_DBCONFIG_RESET_DATABASE, false)?;

                // Execute migrations
                conn.execute_batch(STARTUP_SQL)?;

                Ok::<(), Error>(())
            })
            .await??;

        migration::run(&self.pool).await
    }
}

fn get_event_by_id(conn: &Connection, event_id: [u8; 32]) -> Result<Option<Event>, Error> {
    let mut stmt = conn.prepare_cached(
        "SELECT pubkey, created_at, kind, tags, content, sig FROM events WHERE id = ?;",
    )?;
    let mut rows = stmt.query([event_id])?;
    match rows.next()? {
        Some(row) => {
            let id = EventId::from_byte_array(event_id);

            let pubkey = row.get_ref(0)?.as_bytes()?;
            let pubkey = PublicKey::from_slice(pubkey)?;

            let created_at: i64 = row.get_ref(1)?.as_i64()?;
            let created_at: Timestamp = Timestamp::from_secs(created_at as u64);

            let kind = row.get_ref(2)?.as_i64()?;
            let kind: Kind = Kind::from_u16(kind as u16);

            let tags: Vec<Vec<String>> = row.get(3)?;
            let tags = tags.to_vec();

            let content: String = row.get(4)?;

            let sig = row.get_ref(5)?.as_bytes()?;
            let sig = Signature::from_slice(sig)?;

            Ok(Some(Event::new(
                id, pubkey, created_at, kind, tags, content, sig,
            )))
        }
        None => Ok(None),
    }
}

fn delete_event_by_id(conn: &Connection, event_id: [u8; 32]) -> Result<(), Error> {
    let mut stmt = conn.prepare("DELETE FROM event WHERE id = ?;")?;
    stmt.execute([event_id])?;
    Ok(())
}

// /// Find all events that match the filter
// fn single_filter_query<'a>(
//     conn: &mut Connection,
//     filter: Filter,
// ) -> Result<Box<dyn Iterator<Item = DatabaseEvent<'a>> + 'a>, Error> {
//
// }
