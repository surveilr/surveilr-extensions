use libsqlite3_sys::{sqlite3, sqlite3_api_routines};
use std::os::raw::c_int;

extern "C" {
    pub fn sqlite3_text_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_crypto_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_define_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_fileio_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_fuzzy_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_ipaddr_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_math_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_regexp_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_stats_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_time_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_unicode_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_uuid_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;

    pub fn sqlite3_vsv_init(
        db: *mut sqlite3,
        pzErrmsg: *mut *mut ::std::os::raw::c_char,
        pApi: *const sqlite3_api_routines,
    ) -> c_int;
}
