mod parser;
mod structs;
mod errors;
mod utils;
mod modules {
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use serde::{Serialize, Deserialize};
    pub use super::structs::Node;
    pub use super::errors::IOError;
    pub use super::parser::parse;
    pub use super::utils::*;
}

// use modules::*;

fn main() {
    env_logger::init();
}


#[cfg(test)]
mod test {
    // use super::*;
    use super::modules::*;
    pub use serde_json::from_str;

    fn init_log() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_read_file_to_string() {
        #[derive(Deserialize)]
        struct TestCase {
            enabled: u8,
            source: String,
            expected: String,
        }

        init_log();
        let cases: Vec<TestCase> = from_str(&read_file_to_string("test_data/cases.json").unwrap()).unwrap();
        let case_count = cases.len();
        let mut test_count = 0;
        for case in cases {
            if case.enabled == 0 {
                continue;
            }
            log::debug!("[TEST] source: {}", &case.source);
            let source = read_file_to_string(&case.source).unwrap();
            let scene = clean_up_scene(source, case.source);
            let expected = read_file_to_string(&case.expected).unwrap();
            assert_eq!(scene, expected);
            test_count += 1;
        }
        assert_eq!(test_count, case_count);
    }
}
