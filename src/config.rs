use crate::modules::*;

#[derive(Debug)]
pub struct Config {
    script: String,
    ignore_commands: bool,
    max_body_lines: usize,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
}

impl Config {
    fn new(path: String) -> Self {
        Self {
            script: path,
            ignore_commands: false,
            max_body_lines: 1000,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
        }
    }

    pub fn get_script(&self) -> String {
        self.script.clone()
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
    if matches.get_one::<String>("ignorecmd").is_some() {
        config.ignore_commands = true;
    }
    if let Some(lines) = matches.get_one::<u16>("maxbodylines") {
        config.max_body_lines = *lines as usize;
    }
    if matches.get_one::<String>("emptynodes").is_some() {
        config.write_empty_ignored_nodes = true;
    }
    if let Some(ignoretypes) = matches.get_many::<String>("ignoretypes") {
        config.ignore_node_types = ignoretypes.map(|a| a.to_string()).collect::<Vec<String>>();
    }
    config
}
