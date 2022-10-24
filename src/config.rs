use crate::modules::*;

#[derive(Debug)]
pub struct Config {
    script: String,
    ignore_commands: bool,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
    max_body_lines: usize,
}

impl Config {
    fn new(path: String) -> Self {
        Self {
            script: path,
            ignore_commands: false,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
            max_body_lines: 1000,
        }
    }
}

pub fn get_config() -> Config {
    let matches = Command::new("nukedoctor")
        .name("nukedoctor")
        .about("about stuff alksdfhj laskdfh alkjdfh aksdhf")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("scene")
            .required(true),
            Arg::new("ignorecmd")
            .help("ignore commands.")
            .required(false)
        ])
        .get_matches();

    let scene = matches.get_one::<String>("scene").unwrap().to_owned();
    let mut config = Config::new(scene);
    if matches.get_one::<String>("ignorecmd").is_some() {
        config.ignore_commands = true;
    }
    config
}
