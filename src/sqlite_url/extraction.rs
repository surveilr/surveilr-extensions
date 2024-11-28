use rusqlite::{functions::FunctionFlags, Connection, Error, Result};
use url::Url;

use super::UserError;

pub fn register_extraction_functions(conn: &Connection) -> Result<()> {
    conn.create_scalar_function(
        "url",
        -1,
        rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            let args_count = ctx.len();
            if args_count < 1 {
                return Err(Error::UserFunctionError(Box::new(UserError(
                    "url() requires at least one argument".to_string(),
                ))));
            }

            let base_url = ctx.get_raw(0).as_str().unwrap_or("").to_string();
            let mut url = if !base_url.is_empty() {
                Url::parse(&base_url).map_err(|e| {
                    Error::UserFunctionError(Box::new(UserError(format!(
                        "Invalid base URL: {}",
                        e
                    ))))
                })?
            } else {
                Url::parse("https://").unwrap()
            };

            for i in (1..args_count).step_by(2) {
                let key = ctx.get_raw(i).as_str().unwrap_or("");
                let value = ctx.get_raw(i + 1).as_str().unwrap_or("");
                match key {
                    "scheme" => {
                        url.set_scheme(value).map_err(|_| {
                            Error::UserFunctionError(Box::new(UserError(
                                "Invalid scheme".to_string(),
                            )))
                        })?;
                    }
                    "host" => {
                        url.set_host(Some(value)).map_err(|_| {
                            Error::UserFunctionError(Box::new(UserError(
                                "Invalid host".to_string(),
                            )))
                        })?;
                    }
                    "path" => {
                        url.set_path(value);
                    }
                    "query" => {
                        url.set_query(Some(value));
                    }
                    "fragment" => {
                        url.set_fragment(Some(value));
                    }
                    "user" => {
                        url.set_username(value).map_err(|_| {
                            Error::UserFunctionError(Box::new(UserError(
                                "Invalid username".to_string(),
                            )))
                        })?;
                    }
                    "password" => {
                        url.set_password(Some(value)).map_err(|_| {
                            Error::UserFunctionError(Box::new(UserError(
                                "Invalid password".to_string(),
                            )))
                        })?;
                    }
                    "options" => {
                        let mut path = url.path().to_string();
                        if !path.ends_with(";") {
                            path.push(';');
                        }
                        path.push_str(value);
                        url.set_path(&path);
                    }
                    // TODO
                    "zoneid" => {}
                    _ => {
                        return Err(Error::UserFunctionError(Box::new(UserError(format!(
                            "Unknown key: {}",
                            key
                        )))));
                    }
                }
            }

            Ok(url.to_string())
        },
    )?;
    conn.create_scalar_function(
        "url_valid",
        1,
        rusqlite::functions::FunctionFlags::SQLITE_DETERMINISTIC,
        move |ctx| {
            let url = ctx.get_raw(0).as_str().unwrap_or("");
            let is_valid = Url::parse(url).is_ok();
            Ok(is_valid as i32)
        },
    )?;

    conn.create_scalar_function("url_host", 1, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let url_text: String = ctx.get(0)?;
        let parsed_url =
            Url::parse(&url_text).map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
        Ok(parsed_url.host_str().unwrap_or("").to_string())
    })?;

    conn.create_scalar_function("url_path", 1, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let url_text: String = ctx.get(0)?;
        let parsed_url =
            Url::parse(&url_text).map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
        Ok(parsed_url.path().to_string())
    })?;

    conn.create_scalar_function(
        "url_scheme",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let url_text: String = ctx.get(0)?;
            let parsed_url = Url::parse(&url_text)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
            Ok(parsed_url.scheme().to_string())
        },
    )?;

    conn.create_scalar_function("url_query", 1, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let url_text: String = ctx.get(0)?;
        let parsed_url =
            Url::parse(&url_text).map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
        Ok(parsed_url.query().unwrap_or("").to_string())
    })?;

    conn.create_scalar_function(
        "url_fragment",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let url_text: String = ctx.get(0)?;
            let parsed_url = Url::parse(&url_text)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
            Ok(parsed_url.fragment().unwrap_or("").to_string())
        },
    )?;

    conn.create_scalar_function("url_user", 1, FunctionFlags::SQLITE_DETERMINISTIC, |ctx| {
        let url_text: String = ctx.get(0)?;
        let parsed_url =
            Url::parse(&url_text).map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
        Ok(parsed_url.username().to_string())
    })?;

    conn.create_scalar_function(
        "url_password",
        1,
        FunctionFlags::SQLITE_DETERMINISTIC,
        |ctx| {
            let url_text: String = ctx.get(0)?;
            let parsed_url = Url::parse(&url_text)
                .map_err(|err| rusqlite::Error::UserFunctionError(err.into()))?;
            Ok(parsed_url.password().unwrap_or("").to_string())
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
        register_extraction_functions(&conn).unwrap();
        conn
    }

    #[test]
    fn test_url_function() -> rusqlite::Result<()> {
        let conn = setup_connection();

        let result: String = conn.query_row(
            "SELECT url('http://github.com', 'path', 'asg017/sqlite-url', 'fragment', 'usage')",
            [],
            |row| row.get(0),
        )?;
        assert_eq!(result, "http://github.com/asg017/sqlite-url#usage");

        Ok(())
    }

    #[test]
    fn test_url_valid_function() -> rusqlite::Result<()> {
        let conn = setup_connection();

        let result: i32 = conn.query_row("SELECT url_valid('https://google.com')", [], |row| {
            row.get(0)
        })?;
        assert_eq!(result, 1);

        let result: i32 = conn.query_row("SELECT url_valid('invalid')", [], |row| row.get(0))?;
        assert_eq!(result, 0);

        Ok(())
    }

    #[test]
    fn test_url_host() {
        let conn = setup_connection();
        let result: String = conn
            .query_row("SELECT url_host('https://example.com/path')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "example.com");

        let result = conn.query_row("SELECT url_host('invalid')", [], |row| {
            row.get::<_, String>(0)
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_url_path() {
        let conn = setup_connection();
        let result: String = conn
            .query_row(
                "SELECT url_path('https://example.com/path/to/resource')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "/path/to/resource");

        let result: String = conn
            .query_row("SELECT url_path('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "/");
    }

    #[test]
    fn test_url_scheme() {
        let conn = setup_connection();
        let result: String = conn
            .query_row("SELECT url_scheme('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "https");

        let result: String = conn
            .query_row("SELECT url_scheme('ftp://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "ftp");
    }

    #[test]
    fn test_url_query() {
        let conn = setup_connection();
        let result: String = conn
            .query_row(
                "SELECT url_query('https://example.com/path?query=123')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "query=123");

        let result: String = conn
            .query_row("SELECT url_query('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_url_fragment() {
        let conn = setup_connection();
        let result: String = conn
            .query_row(
                "SELECT url_fragment('https://example.com#section')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "section");

        let result: String = conn
            .query_row("SELECT url_fragment('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_url_user() {
        let conn = setup_connection();
        let result: String = conn
            .query_row("SELECT url_user('https://user@example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "user");

        let result: String = conn
            .query_row("SELECT url_user('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_url_password() {
        let conn = setup_connection();
        let result: String = conn
            .query_row(
                "SELECT url_password('https://user:pass@example.com')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "pass");

        let result: String = conn
            .query_row("SELECT url_password('https://example.com')", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(result, "");
    }
}
