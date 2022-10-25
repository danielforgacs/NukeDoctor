use crate::modules::*;

#[derive(Debug)]
pub struct Config {
    scene_file: String,
    ignore_commands: bool,
    max_body_lines: Option<usize>,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
}

impl Config {
    pub fn new(path: String) -> Self {
        Self {
            scene_file: path,
            ignore_commands: false,
            max_body_lines: Option::None,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
        }
    }

    pub fn get_scene_file(&self) -> String {
        self.scene_file.clone()
    }

    pub fn get_ignore_node_types(&self) -> &Vec<String> {
        &self.ignore_node_types
    }

    pub fn get_max_body_lines(&self) -> &Option<usize> {
        &self.max_body_lines
    }
}

pub fn get_config() -> Config {
    let matches = Command::new("nukedoctor")
        .name("nukedoctor")
        .about("about stuff alksdfhj laskdfh alkjdfh aksdhf")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("script")
            .required(true),
            Arg::new("ignorecmd")
            .help("ignore commands.")
            .required(false),
            Arg::new("maxbodylines")
            .help("Ignore nodes with more line than this.")
            .value_parser(clap::value_parser!(u16).range(2..))
            .short('l'),
            Arg::new("emptynodes")
            .help("Write ignored nodes empty."),
            Arg::new("ignoretypes")
            .short('i')
            .help("Ignore these node types.")
            .num_args(1..)
        ])
        .get_matches();

    let script = matches.get_one::<String>("script").unwrap().to_owned();
    let mut config = Config::new(script);
    if let Some(ignorecmd) = matches.get_one::<String>("ignorecmd") {
        if ignorecmd != "ignorecmd" {
            panic!("wront ignorecmd arg.")
        }
        config.ignore_commands = true;
    }
    if let Some(lines) = matches.get_one::<u16>("maxbodylines") {
        config.max_body_lines = Some(*lines as usize);
    }
    if let Some(writeempty) = matches.get_one::<String>("emptynodes") {
        if writeempty != "writeempty" {
            panic!("wront writeempty arg.")
        }
        config.write_empty_ignored_nodes = true;
    }
    if let Some(ignoretypes) = matches.get_many::<String>("ignoretypes") {
        config.ignore_node_types = ignoretypes.map(|a| a.to_string()).collect::<Vec<String>>();
    }
    config
}
