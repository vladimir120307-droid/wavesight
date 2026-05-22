//! SQLite schema and migrations.

use rusqlite::Connection;

/// Current schema version. Bump on every migration step.
pub const SCHEMA_VERSION: u32 = 2;

/// Apply migrations idempotently up to [`SCHEMA_VERSION`].
///
/// # Errors
/// Propagates any SQLite error.
pub fn migrate(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        r"
        CREATE TABLE IF NOT EXISTS schema_meta (
            key   TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        ",
    )?;

    let current: u32 = conn
        .query_row(
            "SELECT value FROM schema_meta WHERE key = 'version'",
            [],
            |row| {
                row.get::<_, String>(0)?.parse().map_err(|_| {
                    rusqlite::Error::FromSqlConversionFailure(
                        0,
                        rusqlite::types::Type::Text,
                        Box::new(std::io::Error::new(
                            std::io::ErrorKind::InvalidData,
                            "version not an int",
                        )),
                    )
                })
            },
        )
        .unwrap_or(0);

    if current < 1 {
        conn.execute_batch(
            r"
            CREATE TABLE csi_frame (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                node        TEXT    NOT NULL,
                sequence    INTEGER NOT NULL,
                captured_at TEXT    NOT NULL,
                channel     INTEGER NOT NULL,
                rssi_dbm    INTEGER NOT NULL,
                subcarriers TEXT    NOT NULL,
                samples     BLOB    NOT NULL
            );
            CREATE INDEX idx_csi_frame_captured_at ON csi_frame(captured_at);
            CREATE INDEX idx_csi_frame_node_seq    ON csi_frame(node, sequence);
            ",
        )?;
    }

    if current < 2 {
        conn.execute_batch(
            r"
            CREATE TABLE presence_event (
                id          INTEGER PRIMARY KEY AUTOINCREMENT,
                node        TEXT    NOT NULL,
                started_at  TEXT    NOT NULL,
                ended_at    TEXT,
                energy_peak REAL    NOT NULL,
                uncertainty REAL    NOT NULL
            );
            CREATE INDEX idx_presence_event_started_at ON presence_event(started_at);
            ",
        )?;
    }

    conn.execute(
        "INSERT INTO schema_meta(key, value) VALUES('version', ?1)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        rusqlite::params![SCHEMA_VERSION.to_string()],
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_idempotent() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        migrate(&conn).unwrap();
        let v: String = conn
            .query_row(
                "SELECT value FROM schema_meta WHERE key = 'version'",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(v, SCHEMA_VERSION.to_string());
    }

    #[test]
    fn tables_exist() {
        let conn = Connection::open_in_memory().unwrap();
        migrate(&conn).unwrap();
        let count: u32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('csi_frame', 'presence_event', 'schema_meta')",
                [],
                |r| r.get(0),
            )
            .unwrap();
        assert_eq!(count, 3);
    }
}
