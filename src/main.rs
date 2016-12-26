use std::io;
use std::fs::{self, DirEntry,};
use std::path::Path;
use std::fs::File;
use std::io::Write;

extern crate rustc_serialize;
use rustc_serialize::json;

// json encoding: 
#[derive(Debug, RustcEncodable)]
struct Doc {
    path: String,
    filename: String,
}

static ROOT_DIR: &'static str = "/Users/interaction/workspace/temp/testeddocs";

fn main() {
    let target_path = Path::new(ROOT_DIR);
    let mut docs: Box<Vec<Doc>> = Box::new(Vec::new());

    fn create_handler<'a>(docs: &'a mut Box<Vec<Doc>>) -> Box<FnMut(&DirEntry) + 'a> {
        let handler = move |entry: &DirEntry| -> () {
            let rpath_result = entry.path();
            let rpath = match rpath_result.strip_prefix(ROOT_DIR) {
                Ok(path)=> path,
                Err(_)=> panic!("strip failed")
            };
            let doc = Doc {
                path: rpath.to_str().unwrap().to_string(),
                filename: rpath.file_name().unwrap().to_os_string().into_string().unwrap(),
            };
            docs.push(doc);
        };

        Box::new(handler)
    }

    {
        let mut handler = create_handler(&mut docs);
        visit_dirs(&target_path, &mut |entry: &DirEntry|{
            handler(entry)
        }).unwrap();
    }
    save_json(docs.as_ref());
}

fn save_json(result: &Vec<Doc>){
    let json_str = json::encode(&result).unwrap();
    println!("result json is: {}", json_str);
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