use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    nodetype: String,
    name: String,
    body: String,
    body_index: usize,
    body_size: usize,
    group_name: Option<String>
}

impl Node {
    pub fn new(nodetype: String, name: String, body: String, body_index: usize, group_name: Option<String>) -> Self {
        let body_size = body.len();
        Self {
            nodetype,
            name,
            body,
            body_index,
            body_size,
            group_name,
        }
    }

    pub fn to_text(&self) -> String {
        if self.body.len() == 0 {
            return self.nodetype.clone();
        // } else if self.nodetype == "push" {
        } else if ["push", "set"].contains(&self.nodetype.as_str()) {
            return format!(r#"{} {}"#,
                String::from(self.nodetype.clone()),
                self.body,
            );
        }
        format!(r#"{} {{{}}}"#,
            String::from(self.nodetype.clone()),
            self.body,
        )
    }
}
