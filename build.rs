use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let src_dir = Path::new("sqlite3/ext/sqlean/src");
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut build = cc::Build::new();
    build.include("sqlite3/include");
    build.include("sqlite3/ext/sqlean/src");

    // add subdirs for header files
    find_header_dirs(&src_dir, &mut build);

    if cfg!(target_os = "windows") {
        println!("cargo:warning=Excluding sqlite-lines on Windows");
    } else {
        build.include("sqlite3/ext/sqlite-lines");
    }

    // suppress multiple sqlite3_api definitions in the C files
    // (dont really know the implication of these two macros, chatgpt'd it)
    build.define("SQLITE_CORE", None);
    build.define("SQLITE_API_VAR", None);

    // sqlean PCRE2 headers
    build.define("HAVE_CONFIG_H", None);
    build.define("PCRE2_CODE_UNIT_WIDTH", "8");
    build.define("PCRE2_STATIC", None);
    
    build.define("LINK_SIZE", "2");
    build.define("SUPPORT_UNICODE", None); 


    // sqlite-url variables
    build
        .define("SQLITE_LINES_VERSION", "\"v0.1.0\"")
        .define("SQLITE_LINES_DATE", "\"2024-11-28T11:36:54Z\"")
        .define(
            "SQLITE_LINES_SOURCE",
            "\"19cf842b1a5f44a9c23ad0d396d167f004c6eb7f\"",
        );

    // handle BYTE_ORDER definition for Windows
    if cfg!(target_os = "windows") {
        build.define("LITTLE_ENDIAN", Some("1234"));
        build.define("BIG_ENDIAN", Some("4321"));
        build.define("BYTE_ORDER", Some("LITTLE_ENDIAN"));
    }

    find_c_files(&src_dir, &mut build);
    if cfg!(target_os = "windows") {
        println!("cargo:warning=Excluding sqlite-lines on Windows");
    } else {
        build.file("sqlite3/ext/sqlite-lines/sqlite-lines.c");
    }

    build.compile("sqlite3ext");

    println!("cargo:rustc-link-lib=static=sqlite3ext");
    println!("cargo:rustc-link-search=native={}", out_dir);
}

fn find_c_files(dir: &Path, build: &mut cc::Build) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            find_c_files(&path, build);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("c") {
            println!("cargo:rerun-if-changed={}", path.display());
            build.file(path);
        }
    }
}

fn find_header_dirs(dir: &Path, build: &mut cc::Build) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            if contains_header_files(&path) {
                build.include(&path);
            }
            find_header_dirs(&path, build);
        }
    }
}

fn contains_header_files(dir: &Path) -> bool {
    fs::read_dir(dir).unwrap().any(|entry| {
        entry
            .unwrap()
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            == Some("h")
    })
}
