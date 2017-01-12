use rustc_serialize::json;


// json encoding: 
#[derive(Debug, RustcEncodable)]
pub struct Doc {
    pub path: String,
    pub filename: String,
}
