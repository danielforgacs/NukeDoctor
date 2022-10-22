#[derive(Debug)]
pub struct IOError {
    msg: String,
}

impl IOError {
    pub fn new<T: ToString>(msg: T) -> Self {
        Self { msg: msg.to_string() }
    }
}
