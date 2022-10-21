use crate::modules::*;

pub fn clean_up_scene(scene: String) -> String {
    let source: Vec<char> = scene.chars().collect();
    let nodes = parse(source);
    nodes_to_scene(nodes)
}

fn nodes_to_scene(nodes: Vec<Node>) -> String {
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
