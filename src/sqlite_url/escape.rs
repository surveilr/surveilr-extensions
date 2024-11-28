use rusqlite::{Connection, Result, functions::FunctionFlags};
use percent_encoding::{percent_decode, utf8_percent_encode, NON_ALPHANUMERIC};

pub fn register_escape_functions(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "url_escape",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let input: String = ctx.get(0)?;
            let escaped = utf8_percent_encode(&input, NON_ALPHANUMERIC).to_string();
            Ok(escaped)
        },
    )?;

    conn.create_scalar_function(
        "url_unescape",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let input: String = ctx.get(0)?;
            let unescaped = percent_decode(input.as_bytes())
                .decode_utf8()
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
            Ok(unescaped.to_string())
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
        register_escape_functions(&conn).unwrap();
        conn
    }

    #[test]
    fn test_url_escape() {
        let conn = setup_connection();
        let result: String = conn.query_row("SELECT url_escape('hello world')", [], |row| row.get(0)).unwrap();
        assert_eq!(result, "hello%20world");

        let result: String = conn.query_row("SELECT url_escape('special@chars!')", [], |row| row.get(0)).unwrap();
        assert_eq!(result, "special%40chars%21");
    }

    #[test]
    fn test_url_unescape() {
        let conn = setup_connection();
        let result: String = conn.query_row("SELECT url_unescape('hello%20world')", [], |row| row.get(0)).unwrap();
        assert_eq!(result, "hello world");

        let result: String = conn.query_row("SELECT url_unescape('special%40chars%21')", [], |row| row.get(0)).unwrap();
        assert_eq!(result, "special@chars!");
    }
}