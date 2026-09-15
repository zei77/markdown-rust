mod lexer;
mod chunker;

use std::fs;

fn main() -> std::io::Result<()> {

    let text = fs::read_to_string("data/rust_guide.md")?;
    let tokens = lexer::lex(&text);
    let chunks = chunker::chunk(&tokens);

    println!("{}", serde_json::to_string_pretty(&chunks).unwrap());
    Ok(())
}
