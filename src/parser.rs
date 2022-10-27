// use crate::modules::*;
// use nukedoctor::project_modules::*;
// use nukedoctor::structs::Node;
use crate::structs::Node;

pub fn parse(source: Vec<char>) -> Vec<Node> {
    let mut nodes = Vec::with_capacity(5000);
    let mut index = 0;
    let mut group: Option<String> = Option::None;
    loop {
        let char = source[index];
        if char.is_alphabetic() {
            let word = extract_word(&source, &mut index);
            log::debug!("found word: {}, index: {}", &word, &index);
            if source[index..=index + 1] == [' ', '{'] {
                nodes.push(crate_node(word, &mut group, &source, &mut index))
            } else if word == "end_group" {
                group = Option::None;
                nodes.push(Node::new(
                    word.clone(),
                    "".to_string(),
                    "".to_string(),
                    index,
                    group.clone(),
                ));
            } else if word == "push" {
                index += 1;
                let body_index = index;
                let mut push_arg = String::with_capacity(100);
                loop {
                    let char = source[index];
                    if char.is_whitespace() {
                        break;
                    }
                    push_arg.push(char);
                    index += 1;
                    if index == source.len() - 1 {
                        break;
                    }
                }
                nodes.push(Node::new(
                    word,
                    "".to_string(),
                    push_arg,
                    body_index,
                    group.clone(),
                ))
            } else if word == "set" {
                index += 1;
                let body_index = index;
                let arg1 = extract_word(&source, &mut index);
                index += 1;
                let mut arg2 = String::with_capacity(100);
                loop {
                    let char = source[index];
                    if char == '\n' {
                        break;
                    }
                    arg2.push(char);
                    index += 1;
                    if index == source.len() - 1 {
                        break;
                    }
                }
                let body = format!("{} {}", arg1, arg2);
                nodes.push(Node::new(
                    word,
                    "".to_string(),
                    body,
                    body_index,
                    group.clone(),
                ))
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

fn parse_brackets(source: &Vec<char>, index: &mut usize) -> (String, String) {
    let mut body = String::with_capacity(10000);
    let mut name = String::with_capacity(100);
    let mut nesting = 1;
    *index += 1;
    loop {
        let char = source[*index];
        if char.is_alphanumeric() {
            let word = extract_word(source, index);
            body += word.as_str();
            *index -= 1;
            if word == "name" && source[*index + 1].is_whitespace() {
                body.push(' ');
                *index += 1;
                *index += 1;
                name = extract_word(source, index);
                body += name.as_str();
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

fn crate_node(
    word: String,
    group: &mut Option<String>,
    source: &Vec<char>,
    index: &mut usize,
) -> Node {
    skip_whitespace(source, index);
    let body_index = *index + 1;
    let (name, body) = parse_brackets(source, index);
    log::debug!("extracted body. index: {}", &index);
    let node = Node::new(word.clone(), name, body, body_index, group.clone());
    if word == "Group" {
        *group = Option::Some(word);
    } else if word == "end_group" {
        *group = Option::None;
    }
    node
}
