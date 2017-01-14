use std::fs::{self, DirEntry, File};
use std::io::Result;
use std::io::Read;
use std::path::Path;
use std::collections::HashMap;

use regex::Regex;

use models::Doc;

pub fn read(entry: &DirEntry, root_path: &Path) -> Result<Doc> {
    let filepath: &Path = &entry.path();
    let path: String = path_to_string!(filepath.strip_prefix(root_path).unwrap());
    let filename: String = filepath.file_name().unwrap().to_str().unwrap().to_string();
    let mut f = File::open(filepath)?;
    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => {
            let re = Regex::new(r"(?s)^={3,}\n(.*?)={3,}").unwrap();
            if let Some(cap) = re.captures(&s) {

                let meta_ref: &str = cap.get(1).unwrap().as_str();
                let tokens: Vec<(&str, &str)> = meta_ref.lines()
                    .map(|line| {
                        let (key, value) = line.split_at(line.find(':').unwrap());
                        (key[1..].trim(), value[1..].trim())
                    })
                    .collect();

                let mut token_map: HashMap<String, String> = HashMap::new();
                for (key, value) in tokens {
                    token_map.insert(key.to_string(), value.to_string());
                }

                Ok(Doc {
                    path: path,
                    filename: filename,
                    meta: Some(token_map),
                })
            } else {
                Ok(Doc {
                    path: path,
                    filename: filename,
                    meta: None,
                })
            }
        }
        Err(e) => Err(e),
    }
}


#[cfg(test)]
mod test_reader {
    use std::path::Path;
    use std::fs::read_dir;
    use std::fmt::format;

    use super::read;
    use models::Doc;
    use utils::file_to_direntry;


    #[test]
    fn test_read() {
        let path_entry = file_to_direntry("./tests/resources/some.txt").unwrap();
        let doc = read(&path_entry, Path::new("./tests")).unwrap();
        assert!(format!("{:?}", doc).contains("meta: Some"));
    }
}
