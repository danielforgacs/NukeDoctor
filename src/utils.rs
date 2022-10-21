use crate::modules::*;

pub fn clean_up_scene(scene: String) -> String {
    let source: Vec<char> = scene.chars().collect();
    String::from_iter(source)
}

pub fn read_file_to_string(path: &str) -> Result<String, IOError> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .map_err(|e| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .map_err(|e| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}
