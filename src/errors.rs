#[derive(Debug)]
pub struct IOError {
    msg: String,
}

impl IOError {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
}
