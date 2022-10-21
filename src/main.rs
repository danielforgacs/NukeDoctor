mod modules {
    pub use std::path::Path;
    pub use std::fs::File;
    pub use std::io::prelude::*;
}

use modules::*;

fn main() {}

fn read_file_to_string(path: &str) -> String {
    let mut buf = String::new();
    let mut file_handle = File::open(path)
        .expect(format!("Error opening file for reading: {}", path).as_str());
    let _read_bytes = file_handle.read_to_string(&mut buf)
        .expect(format!("Error reading file: {}", path).as_str());
    buf
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read_file_to_string() {
        let expected = read_file_to_string("test_data/scene01_expected.nk");
        let data = read_file_to_string("test_data/scene01.nk");
        assert_eq!(data, expected);
    }

}
