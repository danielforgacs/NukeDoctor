struct Config {
    ignore_commands: bool,
    write_empty_ignored_nodes: bool,
    ignore_node_types: Vec<String>,
    max_body_lines: usize,
}

impl Config {
    fn new() -> Self {
        Self {
            ignore_commands: false,
            write_empty_ignored_nodes: false,
            ignore_node_types: Vec::new(),
            max_body_lines: 1000,
        }
    }
}
