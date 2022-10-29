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
        Ok(_) => {}
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
    use assert_cmd::Command;
    use nukedoctor::project_modules::*;
    pub use serde_json::from_str;

    const TEST_CASES: &str = "test_data/cases.json";

    fn init_log() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn runner(case_name: &str) {
        #[derive(Debug, Deserialize)]
        struct TestCase {
            nocmd: Option<String>,
            maxbodylines: Option<String>,
            emptynodes: Option<String>,
            ignoretypes: Option<String>,
            empty_groups: Option<String>,
        }
        init_log();
        use std::collections::HashMap;
        let cases =
            from_str::<HashMap<String, TestCase>>(&read_file_to_string(TEST_CASES).unwrap())
                .unwrap();
        let case_args = cases.get(case_name).unwrap();
        let source_file = format!("test_data/{}", case_name);
        let result_file = format!("test_data/{}.doctored", case_name);
        let expacted_file = format!("test_data/{}.expected", case_name);
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.arg(source_file.clone());
        if case_args.nocmd.is_some() {
            cmd.arg("-c");
        }
        if let Some(lines) = &case_args.maxbodylines {
            cmd.arg("-l").arg(lines);
        }
        if case_args.emptynodes.is_some() {
            cmd.arg("-e");
        }
        if case_args.empty_groups.is_some() {
            cmd.arg("-g");
        }
        if let Some(ignore_types) = &case_args.ignoretypes {
            cmd.arg("-i").args(ignore_types.split(' '));
        }
        cmd.output().unwrap();
        dbg!(&case_args);
        dbg!(&cmd);
        dbg!(&source_file);
        dbg!(&result_file);
        dbg!(&expacted_file);
        assert_eq!(
            read_file_to_string(result_file.as_str()).unwrap(),
            read_file_to_string(expacted_file.as_str()).unwrap(),
        );
    }

    #[test]
    fn test_no_arg() {
        runner("no_arg");
    }

    #[test]
    fn test_no_dot_w_empty() {
        runner("no_Dot_w_empty");
    }

    #[test]
    fn test_no_dot() {
        runner("no_dot");
    }

    #[test]
    fn test_no_cmd() {
        runner("no_cmd");
    }

    #[test]
    fn test_max_lines() {
        runner("max_lines");
    }

    #[test]
    fn test_multi() {
        runner("multi");
    }

    #[test]
    fn test_empty_groups() {
        runner("empty_groups");
    }
}
