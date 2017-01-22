extern crate rustc_serialize;
extern crate regex;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate glob;

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
    require_literal_leading_dot: false
};

fn main() {
    // -------- start: init env
    dotenv().ok();
    env_logger::init().unwrap();

    let ref doc_path = env::var("TARGET_DOC_PATH").expect("TARGET_DOC_PATH is needed from system env");
    debug!("TARGET_DOC_PATH is: {:?}", doc_path);

    if let Ok(level) = env::var("RUST_LOG") {
        debug!("RUST_LOG env: {}", level);
    }
    // -------- end: init env


    let target_path = Path::new(doc_path);
    let mut docs: Vec<Doc> = Vec::new();
    {
        // move this kind of config to file
        let blacklist: Vec<glob::Pattern> = vec![
            glob::Pattern::new("**/.*").unwrap()
        ];

        let whitelist: Vec<glob::Pattern> = vec![
            glob::Pattern::new("**/*.txt").unwrap(),
            glob::Pattern::new("**/*.md").unwrap(),
        ];

        let mut handler = create_handler(&mut docs, doc_path);
        visit_dirs(&target_path, &mut |entry: &DirEntry| handler(entry), &blacklist, &whitelist).unwrap();
    }
    docs.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    save_json(&docs);
}


fn create_handler<'a>(docs: &'a mut Vec<Doc>,
                      root_path: &'a str)
                      -> Box<FnMut(&DirEntry) + 'a> {
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
fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry), excludes: &Vec<glob::Pattern>, includes: &Vec<glob::Pattern>) -> io::Result<()> {
    if fs::metadata(dir)?.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;

            let mut is_exclude = false;
            for ref pattern in excludes {
                // self.pattern.matches_path_with(Path::new(&s), &options),
                if pattern.matches_path_with(&entry.path(), &GLOB_OPTIONS) {
                    is_exclude = true;
                    break;
                }
            }

            if is_exclude {
                debug!("skip direntry in black list: {:?}", &entry.path());
                continue;
            }


            if fs::metadata(entry.path())?.is_dir() {
                visit_dirs(&entry.path(), cb, excludes, includes)?;
            } else {
                let mut is_include = false;
                for ref pattern in includes {
                    if pattern.matches_path_with(&entry.path(), &GLOB_OPTIONS) && !is_include {
                        is_include = true;
                    }
                }

                if !is_include {
                    debug!("skip direntry not in white list: {:?}", &entry.path());
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
