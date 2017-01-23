use toml;
use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::convert::From;
use std::io::Read;

use models::{DocConfig, Config};

pub fn parse<T: AsRef<Path>>(config_path: T) -> Result<toml::Table, Box<Error>> {
    let mut file = File::open(config_path.as_ref())?;
    // Read the file contents into a string, returns `io::Result<usize>`
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return toml::Parser::new(&content).parse().ok_or(From::from("toml parse failed"));
}

pub fn parse_config<T: AsRef<Path>>(config_path: T) -> Config {
    let malformat_config = "malformat config file";
    let table = parse(config_path).expect(malformat_config);
    let doc: &toml::Value = table.get("doc").expect(malformat_config);
    let path = doc.lookup("path").and_then(|e| e.as_str()).expect(malformat_config).to_string();
    let whitelist = doc.lookup("whitelist").and_then(|ls| {
        ls.as_slice().and_then(|ostr| {
            let mut result: Vec<String> = Vec::new();
            for value in ostr {
                let p = value.as_str();
                if p.is_some() {
                    result.push(p.unwrap().to_string());
                } else {
                    return None;
                }
            }
            return Some(result);
        })
    });

    let blacklist = doc.lookup("blacklist").and_then(|ls| {
        ls.as_slice().and_then(|ostr| {
            let mut result: Vec<String> = Vec::new();
            for value in ostr {
                let p = value.as_str();
                if p.is_some() {
                    result.push(p.unwrap().to_string());
                } else {
                    return None;
                }
            }
            return Some(result);
        })
    });

    let doc_config = DocConfig {
        path: path,
        whitelist: whitelist,
        blacklist: blacklist,
    };

    let config = Config { doc: doc_config };

    return config;
}

#[cfg(test)]
mod test_config_parser {
    use super::{parse, parse_config};

    #[test]
    fn test_parse() {
        let result = parse("tests/resources/config.toml");
        match result {
            Ok(table) => {
                let doc = table.get("doc").unwrap();
                // value.lookup("test.foo")
                println!("doc {:?}", doc);
                println!("path {:?}", doc.lookup("whitelist").unwrap());
                let whitelist = doc.lookup("whitelist").unwrap().as_slice().unwrap();
                println!("whitelist {:?}", whitelist);
                println!("whitelist[0] {:?}", whitelist[0]);

                assert!(true);
            }
            _ => {
                assert!(false);
            }
        }
    }


    #[test]
    fn test_parse_config() {
        let config = parse_config("tests/resources/config.toml");
        assert!(config.doc.path == "/Users/interaction/workspace/github/blog");
        assert!(config.doc.whitelist.is_some());
        assert!(config.doc.blacklist.is_some());
    }
}
