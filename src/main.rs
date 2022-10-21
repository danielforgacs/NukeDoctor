mod modules {
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::prelude::*;
    pub use serde::Deserialize;
    pub use serde_json::from_str;
}

use modules::*;

#[derive(Debug)]
struct IOError {
    msg: String,
}

impl IOError {
    fn new(msg: String) -> Self {
        Self { msg }
    }
}

fn main() {}

fn read_file_to_string(path: &str) -> Result<String, IOError> {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .map_err(|e| IOError::new(format!("Error opening file for reading: {}", path)))?;
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .map_err(|e| IOError::new(format!("Error reading file: {}", path)))?;
    Ok(buf)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        #[derive(Deserialize)]
        struct TestCase {
            source: String,
            expected: String,
        }
        let cases: Vec<TestCase> = from_str(&read_file_to_string("test_data/cases.json").unwrap()).unwrap();
        for case in cases {
            let expected = read_file_to_string(&case.source).unwrap();
            let data = read_file_to_string(&case.expected).unwrap();
            assert_eq!(data, expected);
        }
    }
}
