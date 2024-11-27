use libsqlite3_sys::sqlite3_auto_extension;

mod bindings;

pub fn init() {
    unsafe {
        sqlite3_auto_extension(Some(bindings::sqlite3_text_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_crypto_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_define_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_fileio_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_fuzzy_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_ipaddr_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_math_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_stats_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_time_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_unicode_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_uuid_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_vsv_init));
        sqlite3_auto_extension(Some(bindings::sqlite3_regexp_init));
    }
}

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};

    use super::*;

    fn run_sql_file(conn: &Connection, sql_file_path: &str) -> Result<()> {
        let sql = std::fs::read_to_string(sql_file_path).unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let _rows = stmt.query([])?;
        // for _row in rows {}
        Ok(())
    }

    #[test]
    fn text_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/text.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("text functions failed");
    }

    #[test]
    fn crypto_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/crypto.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("crypto functions failed");
    }

    #[test]
    fn define_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/define.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("define functions failed");
    }

    #[test]
    fn fileio_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fileio.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("fileio functions failed");
    }

    #[test]
    fn fuzzy_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fuzzy.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("fuzzy functions failed");
    }

    #[test]
    fn ipaddr_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/ipaddr.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("ipaddr functions failed");
    }

    #[test]
    fn math_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/math.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("math functions failed");
    }

    #[test]
    fn regexp_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/regexp.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("regexp functions failed");
    }

    #[test]
    fn sqlean_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/sqlean.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("sqlean functions failed");
    }

    #[test]
    fn stats_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/stats.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("stats functions failed");
    }

    #[test]
    fn time_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/time.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("time functions failed");
    }

    #[test]
    fn unicode_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/unicode.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("unicode functions failed");
    }

    #[test]
    fn uuid_functions() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let test_file =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/uuid.sql");
        run_sql_file(&conn, test_file.to_str().unwrap()).expect("uuid functions failed");
    }

    // TODO: figure the issue with .shell commands
    // #[test]
    // fn vsv_functions() {
    //     init();
    //     let conn = Connection::open_in_memory().unwrap();

    //     let test_file =
    //         std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vsv.sql");
    //     run_sql_file(&conn, test_file.to_str().unwrap()).expect("vsv functions failed");
    // }
}
