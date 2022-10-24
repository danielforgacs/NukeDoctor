use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    nodetype: String,
    name: String,
    body: String,
    body_index: usize,
    body_size: usize,
    body_lines: usize,
    group_name: Option<String>,
}

impl Node {
    pub fn new(
        nodetype: String,
        name: String,
        body: String,
        body_index: usize,
        group_name: Option<String>,
    ) -> Self {
        let body_size = body.len();
        let body_lines = body.chars().filter(|char| char == &'\n').count();
        Self {
            nodetype,
            name,
            body,
            body_index,
            body_size,
            body_lines,
            group_name,
        }
    }

    pub fn to_text(&self) -> String {
        if self.body.is_empty() {
            return self.nodetype.clone();
        } else if ["push", "set"].contains(&self.nodetype.as_str()) {
            return format!(r#"{} {}"#, self.nodetype.clone(), self.body,);
        } else if self.group_name.is_some() {
            return format!(
                r#" {} {{{}}}"#,
                self.nodetype.clone(),
                self.body,
            );
        }
        format!(
            r#"{} {{{}}}"#,
            self.nodetype.clone(),
            self.body,
        )
    }
}
