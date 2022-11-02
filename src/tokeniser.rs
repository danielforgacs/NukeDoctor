#[derive(Debug, PartialEq, Eq)]
enum Token {
    Eof,
}

fn get_tokens(source: String) -> Vec<Token> {
    vec![Token::Eof]
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
