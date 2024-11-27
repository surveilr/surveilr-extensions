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
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn can_open_database() {
        let conn = Connection::open_in_memory().unwrap();

        let mut stmt = conn.prepare("SELECT 1").unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let value: i64 = row.get(0).unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn can_execute_text_database() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let mut stmt = conn
            .prepare("select text_substring('hello world', 7)")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let value: String = row.get(0).unwrap();
        assert_eq!(value, "world");
    }

    #[test]
    fn can_execute_fuzzy_database() {
        init();
        let conn = Connection::open_in_memory().unwrap();

        let mut stmt = conn
            .prepare("select fuzzy_damlev('awesome', 'aewsme')")
            .unwrap();
        let mut rows = stmt.query([]).unwrap();

        let row = rows.next().unwrap().unwrap();
        let value: String = row.get(0).unwrap();
        assert_eq!(value, "2");
    }
}
