use libsqlite3_sys::{sqlite3, sqlite3_api_routines};
use std::os::raw::c_int;

extern "C" {
    pub fn sqlite3_lines_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;
}