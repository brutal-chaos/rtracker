use rusqlite::{Connection, SQLITE_OPEN_READ_WRITE, SQLITE_OPEN_CREATE,
               SQLITE_OPEN_FULL_MUTEX, SQLITE_OPEN_URI};

/// Using a slightly modified set of rusqlite open flags, open and return a connection
pub fn db_connect(path: &String) -> Connection {
    let flags = { SQLITE_OPEN_READ_WRITE | SQLITE_OPEN_CREATE |
                  SQLITE_OPEN_FULL_MUTEX | SQLITE_OPEN_URI };

    debug!("Connection to {:?} has been established", path);
    Connection::open_with_flags(path, flags).unwrap()
}

// Initialize the database
pub fn db_init(conn: &Connection) {
    conn.execute("
        CREATE TABLE IF NOT EXISTS torrent (
            info_hash   TEXT,
            ip          TEXT,
            port        INTEGER,
            peer_id     TEXT,
            remaining   INTEGER,
            last_active INTEGER,
            PRIMARY KEY (info_hash, ip, port, peer_id)
        );",
        &[]
    ).unwrap();
}

pub fn db_prune(conn: &Connection) {
    conn.execute(
        "DELETE FROM torrent
        WHERE (strftime('%s','now') - last_active) > 1860;",
        &[]
    ).unwrap();
}
