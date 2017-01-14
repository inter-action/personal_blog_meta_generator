use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};
use std::convert::AsRef;
use std::error::Error;
use std::convert::From;

// There's a question raised during writing the code.
//
// http://stackoverflow.com/questions/41585143/cant-figure-out-a-weird-path-behavior-in-rust
//


// convert filepath to DirEntry
pub fn file_to_direntry<T: AsRef<Path>>(filepath: T) -> Result<DirEntry, Box<Error>> {

    let path = filepath.as_ref();
    let pf = path.to_path_buf();
    if !pf.is_file() {
        return Err(From::from("not a file"));
    }

    let parent = pf.parent();

    match parent {
        Some(parent) => {
            let filename = try!(pf.strip_prefix(parent));
            path_to_entry(parent, filename)
        }
        None => path_to_entry(Path::new("."), path),
    }
}

fn path_to_entry<A: AsRef<Path>, B: AsRef<Path>>(path: A,
                                                 filename: B)
                                                 -> Result<DirEntry, Box<Error>> {
    let filename: &Path = filename.as_ref();
    let path: &Path = path.as_ref();
    for entry in try!(read_dir(path)) {
        let entry = try!(entry);
        if entry.path().is_file() && entry.path().ends_with(filename) {
            return Ok(entry);
        }
    }
    Err(From::from("no file found"))
}



#[cfg(test)]
mod test {
    use super::file_to_direntry;
    use std::path::PathBuf;

    #[test]
    fn test_file_to_direntry() {
        let result = file_to_direntry(PathBuf::from("tests/resources/some.txt"));
        match result {
            Ok(_) => {
                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }


    #[test]
    fn test_file_to_direntry_noparent() {
        let result = file_to_direntry(PathBuf::from("./todos.txt"));
        match result {
            Ok(entry) => {
                assert!(true);
            }
            e => {
                assert!(false);
            }
        }
    }


}
