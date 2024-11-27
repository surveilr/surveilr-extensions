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

    // suppress multiple sqlite3_api definitions in the C files
    // (dont really know the implication of these two macros, chatgpt'd it)
    build.define("SQLITE_CORE", None);
    build.define("SQLITE_API_VAR", None);

    // sqlean PCRE2 headers
    build.define("HAVE_CONFIG_H", None); 
    build.define("PCRE2_CODE_UNIT_WIDTH", "8");

    find_c_files(&src_dir, &mut build);

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
    fs::read_dir(dir)
        .unwrap()
        .any(|entry| entry.unwrap().path().extension().and_then(|ext| ext.to_str()) == Some("h"))
}
