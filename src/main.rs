mod errors;
mod parser;
mod structs;
mod utils;
mod config;
mod modules {
    pub use super::errors::IOError;
    pub use super::parser::parse;
    pub use super::structs::Node;
    pub use super::utils::*;
    pub use super::config::{Config, get_config};
    pub use serde::{Deserialize, Serialize};
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use std::path::Path;
    pub use clap::{Command, Arg, ArgAction};
}

use modules::*;

fn main() {
    env_logger::init();
    let config = get_config();
    dbg!(&config);
    let scene = match read_file_to_string(&config.get_scene_file()) {
        Ok(scene) => scene,
        Err(error) => {
            println!("{}:", error.to_string());
            println!("{}", config.get_scene_file());
            return;
        }
    };
    match clean_up_scene(scene, config) {
        Ok(_) => {},
        Err(msg) => {
            println!("{}", msg);
        }
    };
}

#[cfg(test)]
mod test {
    use super::*;
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
        let cases: Vec<TestCase> =
            from_str(&read_file_to_string("test_data/cases.json").unwrap()).unwrap();
        let case_count = cases.len();
        let mut test_count = 0;
        for case in cases {
            if case.enabled == 0 {
                continue;
            }
            log::debug!("[TEST] source: {}", &case.source);
            let source = read_file_to_string(&case.source).unwrap();
            let config = Config::new(case.source.clone());
            let scene = clean_up_scene(source, config);
            let expected = read_file_to_string(&case.expected).unwrap();
            assert_eq!(scene.unwrap(), expected);
            test_count += 1;
        }
        assert_eq!(test_count, case_count);
    }
}
