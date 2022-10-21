use crate::modules::*;

pub fn parse(source: Vec<char>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(5000);
    let mut index = 0;
    loop {
        let char = source[index];
        if char.is_alphabetic() {
            let word = extract_word(&source, &mut index);
            if word == "Dot" {
                nodes.push(Node::new(word));
            }
        }
        index += 1;
        if index == source.len() - 1 {
            break;
        }
    }
    nodes
}

fn extract_word(source: &Vec<char>, index: &mut usize) -> String {
    let mut word = String::with_capacity(80);
    loop {
        let char = source[*index];
        word.push(char);
        if *index + 1 == source.len() - 1 {
            break;
        }
        *index += 1;
        if !source[*index].is_alphanumeric() {
            break;
        }
    }
    word
}
