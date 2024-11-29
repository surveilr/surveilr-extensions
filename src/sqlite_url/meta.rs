use rusqlite::{functions::FunctionFlags, Connection, Result};

pub fn register_meta_functions(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "url_version",
        0,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |_ctx| Ok(env!("CARGO_PKG_VERSION")),
    )?;

    conn.create_scalar_function(
        "url_debug",
        0,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |_ctx| {
            Ok(format!(
                "Version: v{}\nDate: {}\nSource: {}",
                env!("CARGO_PKG_VERSION"),
                chrono::Utc::now().to_rfc3339(),
                "https://github.com/surveilr/surveilr-extensions"
            ))
        },
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        register_meta_functions(&conn).unwrap();
        conn
    }

    #[test]
    fn test_url_version() {
        let conn = setup_connection();
        let result: String = conn.query_row("SELECT url_version()", [], |row| row.get(0)).unwrap();
        assert_eq!(result, env!("CARGO_PKG_VERSION").trim());
    }

    #[test]
    fn test_url_debug() {
        let conn = setup_connection();
        let result: String = conn.query_row("SELECT url_debug()", [], |row| row.get(0)).unwrap();
        assert!(result.contains("Version:"));
        assert!(result.contains("Date: "));
        assert!(result.contains("Source: "));
    }
}