// use nukedoctor::project_modules::*;
// use nukedoctor::structs::Node;
// use crate::structs::Node;
// use nukedoctor::project_modules::*;
// use crate::project_modules::*;
use clap::{Arg, ArgAction};
use clap::Command;

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

    pub fn get_ignore_commands(&self) -> &bool {
        &self.ignore_commands
    }

    pub fn get_write_empty_ignored(&self) -> &bool {
        &self.write_empty_ignored_nodes
    }
}

pub fn get_config() -> Config {
    let matches = Command::new("nukedoctor")
        .name("nukedoctor")
        .about("Tool that rewrites nuke scripts that take too long to open
or even crash. This tool probably does not fix the issue itself,
but helps identifying the node that causes the issue.
The tool does not touch the original file. It saves
a new one next to it with the name postfixed: \".doctored\".

usage example:
$ ./nukedoctor comp_scene_v001.nk --nocmd -l 7 --emptynodes -i Dot NoOp

the scene will be rewritten as:
comp_scene_v001.nk.doctored

The json file is a data dump of the nodes found in the nuke scene.
")
        .version(env!("CARGO_PKG_VERSION"))
        .args([
            Arg::new("script")
            .required(true),
            Arg::new("nocmd")
            .help("Don't write commands. (set, push, ...)")
            .short('c')
            .long("nocmd")
            .action(ArgAction::SetTrue),
            Arg::new("maxbodylines")
            .help("Only write nodes with less lines than <maxbodylines>.")
            .short('l')
            .value_parser(clap::value_parser!(u16).range(0..1000)),
            Arg::new("emptynodes")
            .help("Writes nodes that are filtered out with the name only. Helps keeping the node tree while getting rid of heavy stuff.")
            .short('e')
            .long("emptynodes")
            .action(ArgAction::SetTrue),
            Arg::new("ignoretypes")
            .help("Don't write nodes of these types. (Dot, NoOp, ...)")
            .short('i')
            .num_args(1..)
        ])
        .get_matches();

    let script = matches.get_one::<String>("script").unwrap().to_owned();
    let mut config = Config::new(script);
    if matches.get_flag("nocmd") {
        config.ignore_commands = true;
    }
    if let Some(lines) = matches.get_one::<u16>("maxbodylines") {
        config.max_body_lines = Some(*lines as usize);
    }
    if matches.get_flag("emptynodes") {
        config.write_empty_ignored_nodes = true;
    }
    if let Some(ignoretypes) = matches.get_many::<String>("ignoretypes") {
        config.ignore_node_types = ignoretypes.map(|a| a.to_string()).collect::<Vec<String>>();
    }
    config
}
