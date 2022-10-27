// mod parser;
// mod structs;
// mod utils;
// mod config;
// mod modules {
    // pub use super::parser::parse;
    // pub use super::structs::Node;
    // pub use super::utils::*;
    // pub use super::config::get_config;
// }
mod config;
mod utils;
use config::get_config;
use utils::{read_file_to_string, clean_up_scene};
// use crate::
// use modules::*;
// use nukedoctor::project_modules::*;

fn main() {
    env_logger::init();
    let config = get_config();
    let scene = match read_file_to_string(&config.get_scene_file()) {
        Ok(scene) => scene,
        Err(error) => {
            println!("{}:", error.to_string());
            println!("{}", config.get_scene_file());
            return;
        }
    };
    log::info!("Loaded the scene file.");
    match clean_up_scene(scene, config) {
        Ok(_) => {},
        Err(msg) => {
            println!("{}", msg);
        }
    };
    log::info!("finished.");
    println!("Finished.");
}

#[cfg(test)]
mod test {
    use super::*;
    pub use serde_json::from_str;
    use nukedoctor::project_modules::*;
    use crate::config::Config;


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
