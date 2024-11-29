use libsqlite3_sys::sqlite3_auto_extension;

mod bindings;

pub fn initialize_sqite_lines_extensions() {
    unsafe {
        sqlite3_auto_extension(Some(bindings::sqlite3_lines_init));
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};

    use super::*;

    #[test]
    fn test_lines_version() -> Result<()> {
        initialize_sqite_lines_extensions();
        let conn = Connection::open_in_memory().unwrap();
        let mut stmt = conn.prepare("SELECT lines_version()")?;
        let version: String = stmt.query_row([], |row| row.get(0))?;

        let expected_version = format!("v0.1.0");
        assert_eq!(version, expected_version);
        Ok(())
    }

    #[test]
    fn test_lines_debug() -> Result<()> {
        initialize_sqite_lines_extensions();
        let conn = Connection::open_in_memory().unwrap();
        let mut stmt = conn.prepare("SELECT lines_debug()")?;
        let debug: String = stmt.query_row([], |row| row.get(0))?;
        let debug_lines: Vec<&str> = debug.split('\n').collect();

        assert_eq!(debug_lines.len(), 3);
        assert!(debug_lines[0].starts_with("Version: v"));
        assert!(debug_lines[1].starts_with("Date: "));
        assert!(debug_lines[2].starts_with("Source: "));
        Ok(())
    }

    #[test]
    fn test_lines() -> Result<()> {
        initialize_sqite_lines_extensions();
        let conn = Connection::open_in_memory().unwrap();
        
        let mut stmt = conn.prepare("SELECT rowid, delimiter, document, line FROM lines(?)")?;
        let rows: Vec<(i64, String, String, String)> = stmt
            .query_map(["a\nb"], |row| {
                Ok((
                    row.get(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(
            rows,
            vec![
                (1, "\n".to_string(), "".to_string(), "a".to_string()),
                (2, "\n".to_string(), "".to_string(), "b".to_string()),
            ]
        );

        // let err = {
        //     let mut stmt = conn.prepare("SELECT line FROM lines('axxb', 'xx')")?;
        //     stmt.query([]).err()
        // };
        // println!("{err:#?}");
        // assert!(matches!(err, Some(Error::SqliteFailure(_, _))));

        Ok(())
    }

    #[test]
    fn test_lines_read() -> Result<()> {
        initialize_sqite_lines_extensions();
        let conn = Connection::open_in_memory().unwrap();

        let mut test_files_path = std::env::current_dir().unwrap();
        test_files_path.push("src/sqlite_lines/test_files/test.txt");
    
        let test_files_path_str = test_files_path
            .to_str()
            .expect("Failed to convert test file path to string");

        let mut stmt = conn.prepare("SELECT rowid, path, delimiter, line FROM lines_read(?)")?;
        let rows: Vec<(i64, String, String, String)> = stmt
            .query_map([test_files_path_str], |row| {
                Ok((
                    row.get(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })?
            .collect::<Result<Vec<_>, _>>()?;

        assert_eq!(
            rows,
            vec![
                (1, test_files_path_str.to_string(), "\n".to_string(), "line1".to_string()),
                (2, test_files_path_str.to_string(), "\n".to_string(), "line numba 2".to_string()),
                (3, test_files_path_str.to_string(), "\n".to_string(), "line 3 baby".to_string()),
            ]
        );


        // let err = {
        //     let mut stmt = conn.prepare("SELECT line FROM lines_read('notexist.txt')")?;
        //     stmt.query([]).err()
        // };
        // assert!(matches!(err, Some(Error::SqliteFailure(_, _))));

        Ok(())
    }
}
