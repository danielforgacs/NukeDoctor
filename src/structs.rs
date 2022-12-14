use super::project_modules::*;

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
        let mut body_lines = body.chars().filter(|char| char == &'\n').count();
        if body_lines > 0 {
            body_lines -= 1;
        }
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
            return format!(r#" {} {{{}}}"#, self.nodetype.clone(), self.body,);
        }
        format!(r#"{} {{{}}}"#, self.nodetype.clone(), self.body,)
    }

    pub fn get_nodetype(&self) -> String {
        self.nodetype.clone()
    }

    pub fn get_body_lines(&self) -> &usize {
        &self.body_lines
    }

    pub fn get_group_name(&self) -> &Option<String> {
        &self.group_name
    }

    pub fn set_write_empty_body(&mut self) {
        if !["push", "set"].contains(&self.nodetype.as_ref()) {
            self.body = format!("\n name {}\n", self.name);
        }
    }
}
