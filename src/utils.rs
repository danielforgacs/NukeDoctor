use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
struct NodeBump {
    node_count: usize,
    nodes: Vec<Node>,
}

pub fn clean_up_scene(scene: String, path: String) -> String {
    if scene.len() == 0 {
        return "".to_string();
    }
    let source: Vec<char> = scene.chars().collect();
    log::info!("staring parsing.");
    let nodes = parse(source);
    // DO FILTERING HERE
    // BY
    // - TYPE
    // - NAME
    // - BODY SIZE
    // MARK NODES AS ACTIVE / NOT ACTIVE
    // STILL DUMP NOT ACTOVE
    write_string_to_file(
        &path.replace(".nk", ".json"),
        serde_json::to_string_pretty(&NodeBump {
            node_count: nodes.len(),
            nodes: nodes.clone(),
        }).unwrap()
    ).expect("Can't write node dump json file.");
    nodes_to_scene(&nodes)
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
    scene
}

pub fn read_file_to_string(path: &str) -> Result<String, IOError> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .map_err(|_err| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .map_err(|_err| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}

fn write_string_to_file(path: &str, data: String) -> Result<(), IOError> {
    std::fs::write(path, data).map_err(|_err| IOError::new("Can't dump done json file."))?;
    Ok(())
}
