use serde::Serialize;
use crate::lexer::Token;
use crate::lexer::TokenType;

#[derive(Debug, Clone, Serialize)]
pub struct Chunk {
    pub heading: Vec<String>,
    pub text: String,
    pub start_line: usize,
}

struct ParseState {
    buffer: String,
    heading: Vec<(usize, String)>,
    start_line: usize,
    chunks: Vec<Chunk>,
}

impl ParseState {
    fn new() -> Self {
        Self {
            buffer: String::new(),
            heading: Vec::new(),
            start_line: 0,
            chunks: Vec::new(),
        }
    }

    fn flush(&mut self) {
        if !self.buffer.is_empty() {
            self.chunks.push(Chunk {
                heading: self.heading.iter().map(|(_, h)| h.clone()).collect(),
                text: std::mem::take(&mut self.buffer),
                start_line: self.start_line,
            });
            self.buffer.clear();
        }
    }

    fn set_start_line(&mut self, line: usize) {
        self.start_line = line;
    }
}

pub fn chunk(tokens: &[Token]) -> Vec<Chunk> {
    let mut state = ParseState::new();

    for token in tokens {
        match token.token_type {
            TokenType::Heading => {
                state.flush();
                state.set_start_line(token.line);

                let ParseState { heading, buffer, .. } = &mut state;

                let lvl = token.text
                    .chars()
                    .take_while(|&c| c == '#')
                    .count();

                while heading.len() > 1 && heading.last().unwrap().0 >= lvl {
                    heading.pop();
                }
                heading.push((lvl, token.text.to_string()));

                let parts: Vec<&str> = heading
                    .iter()
                    .map(|(_, h)| h.as_str())
                    .collect();

                buffer.push_str(&parts.join(" > "));
                buffer.push_str("\n\n");
            }

            TokenType::Text => {
                let ParseState { buffer, .. } = &mut state;
                buffer.push_str(&token.text);
            }

            _ => {}
        }
    }

    state.flush();
    state.chunks
}