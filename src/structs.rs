use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub enum NodeType {
    Dot,
    NotNode,
}

#[derive(Debug, Serialize)]
pub struct Node {
    nodetype: NodeType,
    body: String,
}

impl From<String> for NodeType {
    fn from(text: String) -> Self {
        match text.as_str() {
            "Dot" => Self::Dot,
            _ => Self::NotNode,
        }

    }
}

impl From<NodeType> for String {
    fn from(nodetype: NodeType) -> Self {
        match nodetype {
            NodeType::Dot => "Dot".to_string(),
            NodeType::NotNode => "NotNode".to_string(),
        }
    }
}

impl Node {
    pub fn new(nodetype: NodeType, body: String) -> Self {
        Self {
            nodetype,
            body,
        }
    }

    pub fn to_text(&self) -> String {
        format!(r#"{} {{{}}}"#,
            String::from(self.nodetype.clone()),
            self.body,
        )
    }
}
