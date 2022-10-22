use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    nodetype: String,
    body: String,
    body_index: usize,
    body_size: usize,
    group_name: Option<String>
}

impl Node {
    pub fn new(nodetype: String, body: String, body_index: usize, group_name: Option<String>) -> Self {
        let body_size = body.len();
        Self {
            nodetype,
            body,
            body_index,
            body_size,
            group_name,
        }
    }

    pub fn to_text(&self) -> String {
        format!(r#"{} {{{}}}"#,
            String::from(self.nodetype.clone()),
            self.body,
        )
    }
}
