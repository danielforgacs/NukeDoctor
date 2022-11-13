#[derive(Debug, PartialEq, Eq)]
enum Token {
    Eof,
}

fn get_tokens(raw_scene: String) -> Vec<Token> {
    let mut tokens = vec![Token::Eof];
    let source: Vec<char> = raw_scene.chars().collect();
    let mut index = 0;
    while index < source.len() {

    };
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_tokens() {
        let source = r#""#;
        let tokens = get_tokens(source.to_string());
        let expected = vec![
            Token::Eof,
        ];
        assert_eq!(tokens, expected);
    }
}
