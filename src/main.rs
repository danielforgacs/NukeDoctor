use nukedoctor::config::get_config;
use nukedoctor::utils::{clean_up_scene, read_file_to_string};

fn main() {
    env_logger::init();
    let config = get_config();
    let scene = match read_file_to_string(&config.get_scene_file()) {
        Ok(scene) => scene,
        Err(error) => {
            println!("{}:", error);
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
    use assert_cmd::Command;
    use nukedoctor::project_modules::*;

    const TEST_CASES: &str = "test_data/cases.json";

    fn init_log() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn runner(case_name: &str) {
        #[derive(Debug, Deserialize)]
        struct TestCase {
            script: String,
            nocmd: Option<String>,
            maxbodylines: Option<String>,
            emptynodes: Option<String>,
            ignoretypes: Option<String>,
            expected: String,
        }
        init_log();
        use std::collections::HashMap;
        let cases = from_str::<HashMap<String, TestCase>>(&read_file_to_string(TEST_CASES).unwrap()).unwrap();
        let case = cases.get(case_name).unwrap();
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg(case.script.clone());
        if case.nocmd.is_some() {
            cmd.arg("-c");
        }
        if let Some(lines) = &case.maxbodylines {
            cmd.arg("-l");
            cmd.arg(lines);
        }
        if case.emptynodes.is_some() {
            cmd.arg("-e");
        }
        if let Some(ignore_types) = &case.ignoretypes {
            cmd.arg("-i");
            cmd.arg(ignore_types);
        }
        cmd.output().unwrap();
        assert_eq!(
            read_file_to_string(&case.script.clone().as_str()).unwrap(),
            read_file_to_string(&case.expected.clone().as_str()).unwrap()
        );
    }

    #[test]
    fn test_no_arg_run() {
        runner("no_arg");
    }
}
