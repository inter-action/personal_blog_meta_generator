use std::collections::HashMap;

#[derive(Debug, RustcEncodable)]
pub struct Doc {
    pub path: String,
    pub filename: String,
    pub last_modified: u64,
    pub meta: Option<HashMap<String, String>>,
}

// If images should be crafted here, a common File struct need to
// created here in order to abstract code.
// a common Ord trait should be implemented on this File struct.
//
