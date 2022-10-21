use crate::modules::*;

pub fn parse(source: Vec<char>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(5000);
    let mut index = 0;
    loop {
        index += 1;
        if index == source.len() - 1 {
            break;
        }
    }
    nodes
}
