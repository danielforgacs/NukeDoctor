use crate::modules::*;

pub fn parse(source: Vec<char>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(5000);
    let mut index = 0;
    let mut group: Option<String> = Option::None;
    loop {
        let char = source[index];
        if char.is_alphabetic() {
            let word = extract_word(&source, &mut index);
            log::debug!("found word: {}, index: {}", &word, &index);
            if source[index..=index+1] == [' ', '{'] {
                if word == "Group" {
                    group = Option::Some(word.clone());
                } else if word == "end_group" {
                    group = Option::None;
                }
                skip_whitespace(&source, &mut index);
                let body_index = index.clone() + 1;
                let (name, body) = parse_brackets(&source, &mut index);
                log::debug!("extracted body. index: {}", &index);
                nodes.push(Node::new(
                    word,
                    name,
                    body,
                    body_index,
                    group.clone(),
                ));
            }
        }
        index += 1;
        if index >= source.len() - 1 {
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
        if !source[*index].is_alphanumeric() && source[*index] != '_' {
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

fn parse_brackets(source: &Vec<char>, mut index: &mut usize) -> (String, String) {
    let mut body = String::with_capacity(10000);
    let mut name = String::with_capacity(100);
    let mut nesting = 1;
    *index += 1;
    loop {
        let char = source[*index];
        if char.is_alphanumeric() {
            let word = extract_word(&source, &mut index);
            body += &word.as_str();
            *index -= 1;
            if word == "name" {
                body.push(' ');
                *index += 1;
                *index += 1;
                name = extract_word(&source, &mut index);
                body += &name.as_str();
                *index -= 1;
            }
        } else if char == '{' {
            nesting += 1;
            if nesting != 0 {
                body.push(char);
            }
        } else if char == '}' {
            nesting -= 1;
            if nesting == 0 {
                break;
            }
            body.push(char);
        } else {
            body.push(char);
        }
        *index += 1;
        if *index == source.len() - 1 {
            break;
        }
    }
    (name, body)
}
