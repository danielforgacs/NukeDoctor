use crate::modules::*;

pub fn parse(source: Vec<char>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(5000);
    let mut index = 0;
    loop {
        let char = source[index];
        if char.is_alphabetic() {
            let word = extract_word(&source, &mut index);
            if source[index..=index+1] == [' ', '{'] {
                skip_whitespace(&source, &mut index);
                match NodeType::from(word.clone()) {
                    NodeType::NotNode => {},
                    _ => {
                        let body_index = index.clone();
                        let body = parse_brackets(&source, &mut index);
                        nodes.push(Node::new(word.into(), body, body_index));
                    },
                }
            }
        } else {
            index += 1;
        }
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

fn skip_whitespace(source: &Vec<char>, index: &mut usize) {
    loop {
        let char = source[*index];
        if ![' ', '\n'].contains(&char) {
            break;
        }
        *index += 1;
        if *index == source.len() - 1 {
            break;
        }
    }
}

fn parse_brackets(source: &Vec<char>, index: &mut usize) -> String {
    let mut body = String::with_capacity(10000);
    let mut nesting = 1;
    *index += 1;
    loop {
        let char = source[*index];
        if char == '{' {
            nesting += 1;
        }
        if char == '}' {
            nesting -= 1;
            if nesting == 0 {
                break;
            }
        }
        body.push(char);
        *index += 1;
        if *index == source.len() - 1 {
            break;
        }
    }
    body
}
