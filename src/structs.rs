use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub enum NodeType {
    Root,
    ColorWheel,
    Expression,
    Viewer,
    StickyNote,
    Dot,
    UnKnown,
}

#[derive(Debug, Clone, Serialize)]
pub struct Node {
    nodetype: NodeType,
    body: String,
    body_index: usize,
    body_size: usize,
}

impl From<String> for NodeType {
    fn from(text: String) -> Self {
        match text.as_str() {
            "Root" => Self::Root,
            "ColorWheel" => Self::ColorWheel,
            "Expression" => Self::Expression,
            "Viewer" => Self::Viewer,
            "StickyNote" => Self::StickyNote,
            "Dot" => Self::Dot,
            _ => Self::UnKnown,
        }

    }
}

impl From<NodeType> for String {
    fn from(nodetype: NodeType) -> Self {
        match nodetype {
            NodeType::Root => "Root".to_string(),
            NodeType::ColorWheel => "ColorWheel".to_string(),
            NodeType::Expression => "Expression".to_string(),
            NodeType::Viewer => "Viewer".to_string(),
            NodeType::StickyNote => "StickyNote".to_string(),
            NodeType::Dot => "Dot".to_string(),
            NodeType::UnKnown => "NotNode".to_string(),
        }
    }
}

impl Node {
    pub fn new(nodetype: NodeType, body: String, body_index: usize) -> Self {
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
