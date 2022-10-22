mod parser;
mod structs;
mod errors;
mod utils;
mod modules {
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use serde::Deserialize;
    pub use serde_json::from_str;
    pub use super::structs::{Node, NodeType};
    pub use super::errors::IOError;
    pub use super::parser::parse;
    pub use super::utils::*;
}

use modules::*;

fn main() {}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        #[derive(Deserialize)]
        struct TestCase {
            source: String,
            expected: String,
        }
        let cases: Vec<TestCase> = from_str(&read_file_to_string("test_data/cases.json").unwrap()).unwrap();
        for case in cases {
            let source = read_file_to_string(&case.source).unwrap();
            let expected = clean_up_scene(source);
            let data = read_file_to_string(&case.expected).unwrap();
            assert_eq!(data, expected);
        }
    }
}
