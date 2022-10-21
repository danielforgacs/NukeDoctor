use crate::modules::*;

pub fn clean_up_scene(scene: String) -> String {
    if scene.len() == 0 {
        return "".to_string();
    }
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    // dump the nodes to json here.
    nodes_to_scene(nodes)
}

fn nodes_to_scene(nodes: Vec<Node>) -> String {
    let mut scene = String::new();
    for node in nodes {
        scene += &node.to_text();
    }
    String::new()
}

pub fn read_file_to_string(path: &str) -> Result<String, IOError> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .map_err(|e| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .map_err(|e| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}
