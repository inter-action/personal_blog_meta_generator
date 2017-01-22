use std::collections::HashMap;

#[derive(Debug, RustcEncodable)]
pub struct Doc {
    pub path: String,
    pub filename: String,
    pub last_modified: u64,
    pub meta: Option<HashMap<String, String>>,
}
