extern crate rustc_serialize;
extern crate regex;

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

// external
use rustc_serialize::json;

// mine
use models::Doc;

static ROOT_DIR: &'static str = "/Users/interaction/workspace/temp/testeddocs";

fn main() {
    let target_path = Path::new(ROOT_DIR);
    let mut docs: Vec<Doc> = Vec::new();
    {
        let mut handler = create_handler(&mut docs);
        visit_dirs(&target_path, &mut |entry: &DirEntry| handler(entry)).unwrap();
    }
    save_json(&docs);
}

fn create_handler<'a>(docs: &'a mut Vec<Doc>) -> Box<FnMut(&DirEntry) + 'a> {
    let handler = move |entry: &DirEntry| -> () {
        match doc_reader::read(entry, Path::new(ROOT_DIR)) {
            Ok(doc) => {
                docs.push(doc);
            }
            error => {
                println!("failed to read doc {:?}, error: {:?}", entry, error);
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
fn visit_dirs(dir: &Path, cb: &mut FnMut(&DirEntry)) -> io::Result<()> {
    if try!(fs::metadata(dir)).is_dir() {
        for entry in try!(fs::read_dir(dir)) {
            let entry = try!(entry);
            if try!(fs::metadata(entry.path())).is_dir() {
                try!(visit_dirs(&entry.path(), cb));
            } else {
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
