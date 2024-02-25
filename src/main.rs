enum Errors {}

enum TokenType {
    SemiColon,
}

struct Token {
    pub ty: TokenType,
    pub value: String,
}

fn tokenize(code: &str) -> Result<Vec<Token>, Errors> {
    let tokens: Vec<Token> = Vec::new();
    let current_token: Option<Token> = None;
    for c in code.chars() {
        if c == ';' {
            if let Some(tok) = current_token {
                tokens.push(tok);
            }
            tokens.push(Token {
                ty: TokenType::SemiColon,
                value: String::from(";"),
            });

            current_token = None;
        } else if c == '=' {
            if let Some(tok) = current_token {
                tokens.push(tok)
            }
        }
    }
    Ok(tokens)
}

fn main() {
    let tokens = tokenize("x = 2;").unwrap();

    println!("{:?}", tokens)
}
