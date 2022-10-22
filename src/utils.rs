use crate::modules::*;

pub fn clean_up_scene(scene: String) -> String {
    if scene.len() == 0 {
        return "".to_string();
    }
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    // dump the nodes to json here.
    write_string_to_file("node_dump.json", serde_json::to_string_pretty(&nodes).unwrap());
    nodes_to_scene(nodes)
}

fn nodes_to_scene(nodes: Vec<Node>) -> String {
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
        .map_err(|e| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .map_err(|e| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}

fn write_string_to_file(path: &str, data: String) -> Result<(), IOError> {
    std::fs::write(path, data).map_err(|_err| IOError::new("Can't dump done json file."))?;
    Ok(())
}
