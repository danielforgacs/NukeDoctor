#[derive(Debug)]
pub struct Node {
    nodetype: String,
}

impl Node {
    pub fn new(nodetype: String) -> Self {
        Self { nodetype }
    }

    pub fn to_text(&self) -> String {
        let mut node_text = String::with_capacity(10000);
        node_text += self.nodetype.as_ref();
        node_text
    }
}
