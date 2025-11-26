// Example parser implementation demonstrating snippet extraction

use std::collections::HashMap;

// SNIPPET_START: token_definition
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Star,
    Slash,
    LeftParen,
    RightParen,
}
// SNIPPET_END: token_definition

struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    // SNIPPET_START: tokenize_number
    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();

        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_numeric() || ch == '.' {
                num_str.push(ch);
                self.position += 1;
            } else {
                break;
            }
        }

        Token::Number(num_str.parse().unwrap())
    }
    // SNIPPET_END: tokenize_number

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }

    // SNIPPET_START: main_tokenize
    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.position < self.input.len() {
            self.skip_whitespace();

            if self.position >= self.input.len() {
                break;
            }

            let ch = self.input[self.position];

            let token = match ch {
                '+' => { self.position += 1; Token::Plus }
                '-' => { self.position += 1; Token::Minus }
                '*' => { self.position += 1; Token::Star }
                '/' => { self.position += 1; Token::Slash }
                '(' => { self.position += 1; Token::LeftParen }
                ')' => { self.position += 1; Token::RightParen }
                '0'..='9' => self.read_number(),
                _ => panic!("Unexpected character: {}", ch),
            };

            tokens.push(token);
        }

        tokens
    }
    // SNIPPET_END: main_tokenize
}

fn main() {
    let input = "10 + 20 * (3 - 1)";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
