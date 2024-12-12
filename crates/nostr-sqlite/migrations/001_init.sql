-- Database settings
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL; -- Use `FULL` for crash-safe app
PRAGMA foreign_keys = ON;
PRAGMA user_version = 1; -- Schema version

CREATE TABLE IF NOT EXISTS events (
    id BLOB PRIMARY KEY NOT NULL,
    pubkey BLOB NOT NULL,
    created_at INTEGER NOT NULL,
    kind INTEGER NOT NULL,
    tags JSONB NOT NULL,
    content TEXT NOT NULL,
    sig BLOB NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS ididx ON events(id);
CREATE INDEX IF NOT EXISTS pubkeyprefix ON events(pubkey);
CREATE INDEX IF NOT EXISTS timeidx ON events(created_at DESC);
CREATE INDEX IF NOT EXISTS kindidx ON events(kind);
CREATE INDEX IF NOT EXISTS kindtimeidx ON events(kind,created_at DESC);
