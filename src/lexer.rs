#[derive(Debug, Clone, Copy)]
pub enum TokenType { Heading, Fence, Ruler, BlankLine, Text }

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: &'static TokenType,
    pub text: &'a str,
    pub line: usize,
}

fn next_token<'a>(text: &'a str, line: usize) -> Token<'a> {
    let token_type = match text {
        t if t.starts_with("#") => &TokenType::Heading,
        t if t.starts_with("```") || t.starts_with("~~~") => &TokenType::Fence,
        t if t.starts_with("---") => &TokenType::Ruler,
        t if t.trim().is_empty() => &TokenType::BlankLine,
        _ => &TokenType::Text
    };

    Token {
        token_type: token_type,
        text: text,
        line,
    }
}

pub fn lex<'a>(text: &'a str) -> Vec<Token<'a>> {
    text.lines()
        .enumerate()
        .map(|(i, line)| next_token(line, i + 1))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_heading() {
        let text = "# Heading";
        let tokens = lex(text);
        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0].token_type, &TokenType::Heading));
    }
}