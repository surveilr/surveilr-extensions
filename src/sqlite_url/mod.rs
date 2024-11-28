use rusqlite::{Connection, Result};

mod escape;
mod extraction;
mod meta;
mod query_each;

use escape::register_escape_functions;
use extraction::register_extraction_functions;
use meta::register_meta_functions;
use query_each::register_query_each_virtual_table;

#[derive(Debug)]
struct UserError(String);

impl std::fmt::Display for UserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UserError {}

pub fn register_sqlite_url_functions(conn: &Connection) -> Result<()> {
    register_meta_functions(conn)?;
    register_extraction_functions(conn)?;
    register_escape_functions(conn)?;
    register_query_each_virtual_table(conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{params, Connection, Result};

    fn connect() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        register_sqlite_url_functions(&conn)?;
        conn.execute(
            "CREATE TABLE base_functions AS SELECT name FROM pragma_function_list",
            params![],
        )?;
        conn.execute(
            "CREATE TABLE base_modules AS SELECT name FROM pragma_module_list",
            params![],
        )?;

        conn.execute("CREATE TEMP TABLE loaded_functions AS SELECT name FROM pragma_function_list WHERE name NOT IN (SELECT name FROM base_functions) ORDER BY name", params![])?;
        conn.execute("CREATE TEMP TABLE loaded_modules AS SELECT name FROM pragma_module_list WHERE name NOT IN (SELECT name FROM base_modules) ORDER BY name", params![])?;

        Ok(conn)
    }

    fn execute_single_query(conn: &Connection, query: &str) -> Result<String> {
        conn.query_row(query, params![], |row| row.get(0))
    }

    fn execute_single_query_with_param(
        conn: &Connection,
        query: &str,
        param: &str,
    ) -> Result<String> {
        conn.query_row(query, &[&param], |row| row.get(0))
    }

    #[test]
    fn test_url_version() -> Result<()> {
        let conn = connect()?;

        let expected_version = format!("{}", env!("CARGO_PKG_VERSION").trim());
        let version = execute_single_query(&conn, "SELECT url_version()")?;

        assert_eq!(version, expected_version);
        Ok(())
    }

    #[test]
    fn test_url_debug() -> Result<()> {
        let conn = connect()?;
        let debug_output = execute_single_query(&conn, "SELECT url_debug()")?;
        let debug_lines: Vec<&str> = debug_output.split('\n').collect();

        assert_eq!(debug_lines.len(), 3);
        assert!(debug_lines[0].starts_with("Version: v"));
        assert!(debug_lines[1].starts_with("Date: "));
        assert!(debug_lines[2].starts_with("Source: "));
        Ok(())
    }

    #[test]
    fn test_url() -> Result<()> {
        let conn = connect()?;

        let mut stmt = conn.prepare("SELECT url(?)")?;
        let result: String = stmt.query_row(["https://sqlite.org"], |row| row.get(0))?;
        assert_eq!(result, "https://sqlite.org/");

        let mut stmt = conn.prepare("SELECT url(?, ?, ?)")?;
        let result: String = stmt
            .query_row(["https://sqlite.org", "path", "footprint.html"], |row| {
                row.get(0)
            })?;
        assert_eq!(result, "https://sqlite.org/footprint.html");

        Ok(())
    }

    #[test]
    fn test_url_valid() -> Result<()> {
        let conn = connect()?;
        let mut stmt = conn.prepare("SELECT url_valid(?)")?;
        let is_valid: i32 = stmt.query_row(["https://t.me"], |row| row.get(0))?;
        assert_eq!(is_valid, 1);
    
        let mut stmt = conn.prepare("SELECT url_valid(?)")?;
        let is_valid: i32 = stmt.query_row(["not"], |row| row.get(0))?;
        assert_eq!(is_valid, 0);
    
        let mut stmt = conn.prepare("SELECT url_valid(?)")?;
        let is_valid: i32 = stmt.query_row(["wss://kasldjf.c"], |row| row.get(0))?;
        assert_eq!(is_valid, 1);
    
        let mut stmt = conn.prepare("SELECT url_valid(?)")?;
        let is_valid: i32 = stmt.query_row(["http://a"], |row| row.get(0))?;
        assert_eq!(is_valid, 1);
        Ok(())
    }

    #[test]
    fn test_url_escape() -> Result<()> {
        let conn = connect()?;
        let url_escape = |arg: &str| {
            execute_single_query_with_param(&conn, "SELECT url_escape(?)", arg).unwrap()
        };

        assert_eq!(url_escape("alex garcia, &="), "alex%20garcia%2C%20%26%3D");
        Ok(())
    }

    #[test]
    fn test_url_unescape() -> Result<()> {
        let conn = connect()?;
        let url_unescape = |arg: &str| {
            execute_single_query_with_param(&conn, "SELECT url_unescape(?)", arg).unwrap()
        };

        assert_eq!(url_unescape("alex%20garcia%2C%20%26%3D"), "alex garcia, &=");
        Ok(())
    }

    #[test]
    fn test_url_query_each() -> Result<()> {
        let conn = connect()?;

        let query_each = |query: &str| -> Vec<(i64, String, String)> {
            conn.prepare("SELECT rowid, name, value FROM url_query_each(?)")
                .unwrap()
                .query_map(&[query], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
                .unwrap()
                .filter_map(Result::ok)
                .collect()
        };

        let results = query_each("a=b&c=d");
        assert_eq!(
            results,
            vec![
                (0, "a".to_string(), "b".to_string()),
                (1, "c".to_string(), "d".to_string())
            ]
        );
        Ok(())
    }
}
