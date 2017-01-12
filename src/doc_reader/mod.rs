pub mod reader {
    use std::fs::{self, DirEntry};
    use models::Doc;

    pub fn read(entry: &DirEntry)-> Doc{
        Doc {
           path: "".to_string(),
           filename: "".to_string() 
        }
    }
}

#[cfg(test)]
mod test_reader {
    use std::path::Path;
    use std::fs::read_dir;
    use std::fmt::format;

    use super::reader::read;
    use models::Doc;
    use utils::file_to_direntry;


    #[test]
    fn test_create_capacity_zero() {
        let path_entry = file_to_direntry("./tests/resources/some.txt").unwrap();
        let doc = read(&path_entry);
        assert_eq!(format!("{:?}", doc), format!("{:?}", Doc{path: "".to_string(), filename:"".to_string()}));
        assert_eq!(1, 1);
    }
}