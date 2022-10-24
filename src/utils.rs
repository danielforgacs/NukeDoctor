use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
struct NodeBump {
    node_count: usize,
    nodes: Vec<Node>,
}

pub fn clean_up_scene(scene: String, path: String) -> Result<String, String> {
    if scene.is_empty() {
        return Err("The scene is empty.".to_string());
    }
    let source: Vec<char> = scene.chars().collect();
    log::info!("staring parsing.");
    let nodes = parse(source);
    write_string_to_file(
        &format!("{}.json", path),
        serde_json::to_string_pretty(&NodeBump {
            node_count: nodes.len(),
            nodes: nodes.clone(),
        })
        .unwrap(),
    )
    .expect("Can't write node dump json file.");
    // DO FILTERING HERE
    // BY
    // - TYPE
    // - NAME
    // - BODY SIZE
    // MARK NODES AS ACTIVE / NOT ACTIVE
    // STILL DUMP NOT ACTOVE
    let nodes = nodes.into_iter().filter(|n| n.get_nodetype() != "Dot").collect::<Vec<Node>>();
    let scene = nodes_to_scene(&nodes);
    write_string_to_file(&format!("{}.doctored", path), scene.clone())
        .expect("Can't save the doctored scene.");
    Ok(scene)
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

pub fn read_file_to_string(path: &str) -> Result<String, IOError> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .map_err(|_err| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle
        .read_to_string(&mut buf)
        .map_err(|_err| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}

fn write_string_to_file(path: &str, data: String) -> Result<(), IOError> {
    std::fs::write(path, data).map_err(|_err| IOError::new("Can't dump done json file."))?;
    Ok(())
}
