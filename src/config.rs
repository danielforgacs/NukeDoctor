use super::project_modules::*;

#[derive(Debug, PartialEq)]
pub struct Config {
    scene_file: String,
    ignore_commands: bool,
    max_body_lines: Option<usize>,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
    empty_groups: bool,
}

#[derive(Clone)]
pub struct ConfigBuilder {
    scene_file: String,
    ignore_commands: bool,
    max_body_lines: Option<usize>,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
    empty_groups: bool,
}

impl ConfigBuilder {
    pub fn build(path: String) -> Self {
        ConfigBuilder {
                scene_file: path,
                ignore_commands: false,
                max_body_lines: Option::None,
                write_empty_ignored_nodes: false,
                ignore_node_types: Vec::new(),
                empty_groups: false,
            }
        }

    pub fn ignore_commands(&mut self) -> &mut Self {
        self.ignore_commands = true;
        self
    }

    pub fn max_lines(&mut self, lines: usize) -> &mut Self {
        self.max_body_lines = Some(lines);
        self
    }

    pub fn write_ignored(&mut self) -> &mut Self {
        self.write_empty_ignored_nodes = true;
        self
    }

    pub fn ignore_types(&mut self, node_types: Vec<String>) -> &mut Self {
        self.ignore_node_types = node_types;
        self
    }

    pub fn write_empty_groups(&mut self) -> &mut Self {
        self.empty_groups = true;
        self
    }

    pub fn finish(&self) -> Config {
        Config {
            scene_file: self.scene_file.clone(),
            ignore_commands: self.ignore_commands,
            max_body_lines: self.max_body_lines,
            write_empty_ignored_nodes: self.write_empty_ignored_nodes,
            ignore_node_types: self.ignore_node_types.clone(),
            empty_groups: self.empty_groups,
        }
    }
}

impl Config {
    pub fn new(
        scene_file: String,
        ignore_commands: bool,
        max_body_lines: Option<usize>,
        write_empty_ignored_nodes: bool,
        ignore_node_types: Vec<String>,
        empty_groups: bool,
    ) -> Self {
        Self {
            scene_file,
            ignore_commands,
            max_body_lines,
            write_empty_ignored_nodes,
            ignore_node_types,
            empty_groups,
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

    pub fn get_empty_groups(&self) -> &bool {
        &self.empty_groups
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
            Arg::new("emptygroups")
            .help("Writes Groups empty.")
            .short('g')
            .long("emptygroups")
            .action(ArgAction::SetTrue),
            Arg::new("ignoretypes")
            .help("Don't write nodes of these types. (Dot, NoOp, ...)")
            .short('i')
            .num_args(1..)
        ])
        .get_matches();

    let script = matches.get_one::<String>("script").unwrap().to_owned();
    let mut config = ConfigBuilder::build(script);
    if matches.get_flag("nocmd") {
        config.ignore_commands();
    }
    if let Some(lines) = matches.get_one::<u16>("maxbodylines") {
        config.max_lines(*lines as usize);
    }
    if matches.get_flag("emptynodes") {
        config.write_ignored();
    }
    if matches.get_flag("emptygroups") {
        config.write_empty_groups();
    }
    if let Some(ignoretypes) = matches.get_many::<String>("ignoretypes") {
        config.ignore_types(ignoretypes.map(|a| a.to_string()).collect::<Vec<String>>());
    }
    config.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder_start() {
        let config = ConfigBuilder::build("some_path".to_string()).finish();
        let expected = Config {
            scene_file: "some_path".to_string(),
            ignore_commands: false,
            max_body_lines: Option::None,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
            empty_groups: false,
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn test_config_builder_ignore_commands() {
        let config = ConfigBuilder::build("some_path".to_string())
        .ignore_commands()
        .finish();
        let expected = Config {
            scene_file: "some_path".to_string(),
            ignore_commands: true,
            max_body_lines: Option::None,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
            empty_groups: false,
        };
        assert_eq!(config, expected);
    }

    #[test]
    fn test_config_builder_max_lines_write_ignored() {
        let config = ConfigBuilder::build("some_path".to_string())
        .ignore_commands()
        .max_lines(127)
        .write_ignored()
        .write_empty_groups()
        .finish();
        let expected = Config {
            scene_file: "some_path".to_string(),
            ignore_commands: true,
            max_body_lines: Some(127),
            write_empty_ignored_nodes: true,
            ignore_node_types: Vec::new(),
            empty_groups: true,
        };
        assert_eq!(config, expected);
    }
}
