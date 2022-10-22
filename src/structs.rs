#[derive(Debug, Clone)]
pub enum NodeType {
    Dot,
    NotNode,
}

#[derive(Debug)]
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
        let mut node_text = String::with_capacity(10000);
        node_text += String::from(self.nodetype.clone()).as_ref();
        node_text += " {";
        node_text += &self.body;
        node_text.push('}');
        node_text
    }
}
