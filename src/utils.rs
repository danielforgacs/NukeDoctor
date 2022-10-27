use super::project_modules::*;
use crate::structs::Node;
use crate::parser::parse;
use crate::config::Config;

#[derive(Debug, Clone, Serialize)]
struct NodeBump {
    node_count: usize,
    nodes: Vec<Node>,
}

pub fn clean_up_scene(scene: String, config: Config) -> Result<String, String> {
    if scene.is_empty() {
        log::info!("The scene is empty.");
        return Err("The scene is empty.".to_string());
    }
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    log::info!("done parsing.");
    write_string_to_file(
        &format!("{}.json", config.get_scene_file()),
        serde_json::to_string_pretty(&NodeBump {
            node_count: nodes.len(),
            nodes: nodes.clone(),
        }).map_err(|error| error.to_string())?
    ).map_err(|error| error.to_string())?;
    log::info!("Dumped nodes data .json file.");
    let nodes = filter_nodes(nodes, &config);
    log::info!("Done filtering");
    let scene = nodes_to_scene(&nodes);
    write_string_to_file(&format!("{}.doctored", config.get_scene_file()), scene.clone())
        .map_err(|error| error.to_string())?;
    Ok(scene)
}

fn filter_nodes(mut nodes: Vec<Node>, config: &Config) -> Vec<Node> {
    if !config.get_ignore_node_types().is_empty() {
        log::info!("Filtering by node types. {:?}", &config.get_ignore_node_types());
        if !*config.get_write_empty_ignored() {
            nodes = nodes
                .into_iter()
                .filter(|n| !config.get_ignore_node_types().contains(&n.get_nodetype()))
                .collect::<Vec<Node>>();
        } else {
            nodes
            .iter_mut()
            .for_each(|node| {
                if config.get_ignore_node_types().contains(&node.get_nodetype()) {
                    node.set_write_empty_body();
                }
            })
        }

    }
    if let Some(max_lines) = config.get_max_body_lines() {
        log::info!("Filtering by line count: {}.", max_lines);
        if !*config.get_write_empty_ignored() {
            nodes = nodes
                .into_iter()
                .filter(|node| node.get_body_lines() <= max_lines)
                .collect::<Vec<Node>>();
        } else {
            nodes
            .iter_mut()
            .for_each(|node| {
                if node.get_body_lines() > max_lines {
                    node.set_write_empty_body();
                }
            })
        }
    }
    if *config.get_ignore_commands() {
        log::info!("Ignoring commands.");
        nodes = nodes
            .into_iter()
            .filter(|node| !["set", "push"].contains(&node.get_nodetype().as_str()))
            .collect::<Vec<Node>>();
    }
    nodes
}

fn nodes_to_scene(nodes: &Vec<Node>) -> String {
    let mut scene = String::with_capacity(10000);
    let mut is_first = true;
    for node in nodes {
        if !is_first {
            scene.push('\n');
        } else {
            is_first = false;
        }
        scene += &node.to_text();
    }
    scene.push('\n');
    scene
}

pub fn read_file_to_string(path: &str) -> Result<String, std::io::Error> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)?;
    let _read_bytes = file_handle.read_to_string(&mut buf)?;
    Ok(buf)
}

fn write_string_to_file(path: &str, data: String) -> Result<(), std::io::Error> {
    std::fs::write(path, data)?;
    Ok(())
}
