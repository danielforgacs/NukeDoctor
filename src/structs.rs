use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    nodetype: String,
    body: String,
    body_index: usize,
    body_size: usize,
}

impl Node {
    pub fn new(nodetype: String, body: String, body_index: usize) -> Self {
        let body_size = body.len();
        Self {
            nodetype,
            body,
            body_index,
            body_size,
        }
    }

    pub fn to_text(&self) -> String {
        format!(r#"{} {{{}}}"#,
            String::from(self.nodetype.clone()),
            self.body,
        )
    }
}
