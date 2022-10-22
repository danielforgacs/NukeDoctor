use crate::modules::*;

#[derive(Debug, Clone, Serialize)]
pub enum NodeType {
    Root,
    ColorWheel,
    Expression,
    Viewer,
    StickyNote,
    Dot,
    NotNode,
}

#[derive(Debug, Serialize)]
pub struct Node {
    nodetype: NodeType,
    body: String,
    body_index: usize,
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
            _ => Self::NotNode,
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
            NodeType::NotNode => "NotNode".to_string(),
        }
    }
}

impl Node {
    pub fn new(nodetype: NodeType, body: String, body_index: usize) -> Self {
        Self {
            nodetype,
            body,
            body_index,
        }
    }

    pub fn to_text(&self) -> String {
        format!(r#"{} {{{}}}"#,
            String::from(self.nodetype.clone()),
            self.body,
        )
    }
}
