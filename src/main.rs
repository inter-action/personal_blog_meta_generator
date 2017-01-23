extern crate rustc_serialize;
extern crate regex;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate glob;
extern crate toml;

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! path_to_string {
        ($x:expr) => {{
            $x.to_str().unwrap().to_string()
        }};
    }
}

mod models;
mod doc_reader;
mod utils;
mod config_parser;

// std
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::env;

// external
use rustc_serialize::json;
use dotenv::dotenv;

// mine
use models::Doc;

const GLOB_OPTIONS: glob::MatchOptions = glob::MatchOptions {
    case_sensitive: false,
    require_literal_separator: true,
    require_literal_leading_dot: false,
};

fn main() {
    // -------- start: init env
    dotenv().ok();
    env_logger::init().unwrap();

    let ref config_path = env::var("CONFIG_PATH").expect("CONFIG_PATH is needed from system env");
    debug!("CONFIG_PATH is: {:?}", config_path);

    if let Ok(level) = env::var("RUST_LOG") {
        debug!("RUST_LOG env: {}", level);
    }

    let config = config_parser::parse_config(config_path);
    // -------- end: init env
    let target_path = Path::new(&config.doc.path);
    let mut docs: Vec<Doc> = Vec::new();
    {
        let mut blacklist: Vec<glob::Pattern> = Vec::new();
        if let Some(bl) = config.doc.blacklist {
            let mut i = 0;
            while i < bl.len() {
                let entry = &bl[i];
                blacklist.push(glob::Pattern::new(entry).expect(&format!("invalid blacklist format: {}", entry)));
                i = i + 1;
            }
        }

        let mut whitelist: Vec<glob::Pattern> = Vec::new();
        if let Some(bl) = config.doc.whitelist {
            let mut i = 0;
            while i < bl.len() {
                let entry = &bl[i];
                whitelist.push(glob::Pattern::new(entry).expect(&format!("invalid whitelist format: {}", entry)));
                i = i + 1;
            }
        }

        let mut handler = create_handler(&mut docs, &config.doc.path);
        visit_dirs(&target_path,
                   &target_path,
                   &mut |entry: &DirEntry| handler(entry),
                   &blacklist,
                   &whitelist)
            .unwrap();
    }
    docs.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    save_json(&docs);
}


fn create_handler<'a>(docs: &'a mut Vec<Doc>, root_path: &'a str) -> Box<FnMut(&DirEntry) + 'a> {
    // due to visit_dirs function, this entry param should always be file type.
    let handler = move |entry: &DirEntry| -> () {
        match doc_reader::read(entry, Path::new(root_path)) {
            Ok(doc) => {
                docs.push(doc);
            }
            error => {
                debug!("failed to read doc {:?}, error: {:?}", entry, error);
            }
        }
    };

    Box::new(handler)
}


fn save_json(result: &Vec<Doc>) {
    let json_str = json::encode(&result).unwrap();
    let mut f = File::create("foo.json").unwrap();
    f.write_all(json_str.as_bytes()).unwrap();
    f.sync_all().unwrap();
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(root_dir: &Path,
              dir: &Path,
              cb: &mut FnMut(&DirEntry),
              excludes: &Vec<glob::Pattern>,
              includes: &Vec<glob::Pattern>)
              -> io::Result<()> {
    if fs::metadata(dir)?.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let entry_path = entry.path();
            let relative_path = &entry_path.strip_prefix(root_dir).expect("strip_prefix failed");
            let mut is_exclude = false;
            for ref pattern in excludes {
                // self.pattern.matches_path_with(Path::new(&s), &options),
                if pattern.matches_path_with(relative_path, &GLOB_OPTIONS) {
                    is_exclude = true;
                    break;
                }
            }

            if is_exclude {
                debug!("skip direntry in black list: {:?}", relative_path);
                continue;
            }

            if fs::metadata(&entry_path)?.is_dir() {
                visit_dirs(root_dir, &entry_path, cb, excludes, includes)?;
            } else {
                let mut is_include = false;
                for ref pattern in includes {
                    if pattern.matches_path_with(relative_path, &GLOB_OPTIONS) && !is_include {
                        is_include = true;
                    }
                }

                if !is_include {
                    debug!("skip direntry not in white list: {:?}", relative_path);
                    continue;
                }

                cb(&entry);
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod test_reader {
    use std::path::PathBuf;

    #[test]
    fn test_create_capacity_zero() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_macro_path_to_string() {
        let path_str: String = path_to_string!(PathBuf::from("./todos.txt"));
        assert_eq!("./todos.txt", path_str);
    }
}
