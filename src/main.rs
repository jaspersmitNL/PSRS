use clap::Parser;
use lexer::Token;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::vec;

fn is_alpha(c: char) -> bool {
    c.is_alphabetic() || c == '_' || c == '$'
}
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}
fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

pub mod lexer {
    #[derive(Clone, Debug, PartialEq)]
    pub enum TokenType {
        String(String),
        Number(f64),
        Boolean(bool),
        Identifier(String),

        At,
        Equal,
        LeftParen,
        RightParen,
        LeftBrace,
        RightBrace,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Token {
        pub _type: TokenType,
        pub lexeme: String,
        pub line: u32,
    }

    #[derive(Clone, Debug)]
    pub struct Scanner {
        pub source: String,
        pub i: usize,
        pub line: u32,

        pub tokens: Vec<Token>,
    }
}

impl lexer::Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            i: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }
    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            let token = self.scan_token();

            if let Some(token) = token {
                self.tokens.push(token);
            }
        }
    }

    pub fn scan_token(&mut self) -> Option<Token> {
        let c = self.advance();

        match c {
            ' ' => None,
            '\t' => None,
            '\r' => None,
            '\n' => {
                self.line += 1;
                None
            }
            '@' => Some(Token {
                _type: lexer::TokenType::At,
                lexeme: String::from("@"),
                line: self.line,
            }),
            '=' => Some(Token {
                _type: lexer::TokenType::Equal,
                lexeme: String::from("="),
                line: self.line,
            }),
            '{' => Some(Token {
                _type: lexer::TokenType::LeftBrace,
                lexeme: String::from("{"),
                line: self.line,
            }),
            '}' => Some(Token {
                _type: lexer::TokenType::RightBrace,
                lexeme: String::from("}"),
                line: self.line,
            }),
            '(' => Some(Token {
                _type: lexer::TokenType::LeftParen,
                lexeme: String::from("("),
                line: self.line,
            }),
            ')' => Some(Token {
                _type: lexer::TokenType::RightParen,
                lexeme: String::from(")"),
                line: self.line,
            }),

            '"' => {
                let mut text = String::new();

                while self.peek(None) != '"' && !self.is_at_end() {
                    text.push(self.advance());
                }

                if self.is_at_end() {
                    panic!("Unterminated string");
                }

                self.advance();

                Some(Token {
                    _type: lexer::TokenType::String(text.clone()),
                    lexeme: text,
                    line: self.line,
                })
            }

            _ => {
                if is_alpha(c) {
                    let mut text = String::from(c);
                    while is_alphanumeric(self.peek(None)) {
                        text.push(self.advance());
                    }

                    if text == "$true" {
                        return Some(Token {
                            _type: lexer::TokenType::Boolean(true),
                            lexeme: text,
                            line: self.line,
                        });
                    } else if text == "$false" {
                        return Some(Token {
                            _type: lexer::TokenType::Boolean(false),
                            lexeme: text,
                            line: self.line,
                        });
                    }

                    Some(Token {
                        _type: lexer::TokenType::Identifier(text.clone()),
                        lexeme: text,
                        line: self.line,
                    })
                } else if is_digit(c) {
                    let mut text = String::from(c);
                    while is_digit(self.peek(None)) {
                        text.push(self.advance());
                    }

                    Some(Token {
                        _type: lexer::TokenType::Number(text.parse::<f64>().unwrap()),
                        lexeme: text,
                        line: self.line,
                    })
                } else {
                    panic!("Unexpected character: {}", c);
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        self.i >= self.source.len()
    }
    fn peek(&mut self, offset: Option<usize>) -> char {
        let offset = offset.unwrap_or(0);

        if self.i + offset >= self.source.len() {
            return '\0';
        }

        self.source
            .chars()
            .nth(self.i + offset)
            .expect("Unexpected EOF")
    }
    fn advance(&mut self) -> char {
        self.i += 1;
        self.source.chars().nth(self.i - 1).expect("Unexpected EOF")
    }
}

pub mod parser {
    use crate::lexer;

    pub struct Parser {
        pub tokens: Vec<lexer::Token>,
        pub i: usize,
    }
}

impl parser::Parser {
    pub fn new(tokens: Vec<lexer::Token>) -> Self {
        Self { tokens, i: 0 }
    }

    pub fn parse(&mut self) -> serde_json::Value {
        match self.peek(None).expect("Unexpected EOF")._type {
            lexer::TokenType::String(_) => self.parse_string(),
            lexer::TokenType::Number(_) => self.parse_number(),
            lexer::TokenType::Identifier(_) => self.parse_identifier(),
            lexer::TokenType::Boolean(_) => self.parse_identifier(),

            _ => {
                if self.match_tokens(vec![lexer::TokenType::At]) {
                    if self.match_tokens(vec![lexer::TokenType::LeftBrace]) {
                        return self.parse_object();
                    }
                    if self.match_tokens(vec![lexer::TokenType::LeftParen]) {
                        return self.parse_array();
                    }
                }

                panic!("Unexpected token: {:?}", self.peek(None));
            }
        }
    }

    pub fn parse_string(&mut self) -> serde_json::Value {
        let token = &mut self.advance().expect("Unexpected EOF");

        match &token._type {
            lexer::TokenType::String(s) => {
                return serde_json::Value::String(s.clone());
            }
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    pub fn parse_identifier(&mut self) -> serde_json::Value {
        let token = &mut self.advance().expect("Unexpected EOF");

        match &token._type {
            lexer::TokenType::Identifier(s) => {
                return serde_json::Value::String(s.clone());
            }
            lexer::TokenType::Boolean(b) => {
                return serde_json::Value::Bool(*b);
            }
            _ => {
                panic!("Unexpected token: {:?} expected Identifier", token);
            }
        }
    }

    pub fn parse_number(&mut self) -> serde_json::Value {
        let token = &mut self.advance().expect("Unexpected EOF");

        match &token._type {
            lexer::TokenType::Number(n) => {
                return serde_json::Value::Number(serde_json::Number::from_f64(*n).unwrap());
            }
            _ => {
                panic!("Unexpected token: {:?}", token);
            }
        }
    }
    pub fn parse_object(&mut self) -> serde_json::Value {
        let mut object = serde_json::Map::new();

        //if direct } return empty object
        if self.check_token(lexer::TokenType::RightBrace) {
            self.advance();
            println!("empty object");
            return serde_json::Value::Object(object);
        }

        //until } parse key value pairs
        while !self.check_token(lexer::TokenType::RightBrace) {
            let key = self.parse_identifier();

            if let serde_json::Value::String(key) = key {
                if !self.match_tokens(vec![lexer::TokenType::Equal]) {
                    panic!("Expected =");
                }

                let value = self.parse();

                object.insert(key, value);
            }
        }

        self.advance();

        serde_json::Value::Object(object)
    }

    pub fn parse_array(&mut self) -> serde_json::Value {
        let mut array = serde_json::Value::Array(vec![]);
        while !self.check_token(lexer::TokenType::RightParen) {
            array.as_array_mut().unwrap().push(self.parse());
        }

        self.advance();

        return array;
    }

    pub fn peek(&mut self, offset: Option<usize>) -> Option<&lexer::Token> {
        let offset = offset.unwrap_or(0);

        if self.i + offset >= self.tokens.len() {
            return None;
        }

        self.tokens.get(self.i + offset)
    }

    pub fn advance(&mut self) -> Option<&lexer::Token> {
        self.i += 1;
        self.tokens.get(self.i - 1)
    }

    pub fn is_eof(&self) -> bool {
        self.i >= self.tokens.len()
    }

    pub fn check_token(&mut self, token_type: lexer::TokenType) -> bool {
        if self.is_eof() {
            return false;
        }

        if self.peek(None).unwrap()._type == token_type {
            return true;
        }

        false
    }
    pub fn match_tokens(&mut self, token_types: Vec<lexer::TokenType>) -> bool {
        for token_type in token_types {
            if self.check_token(token_type) {
                self.advance(); // Consume the token
                return true;
            }
        }

        false
    }
}

pub mod writer {
    pub struct Writer {
        pub indent: usize,
        pub root: serde_json::Value, // root value
    }

    pub fn padding(num: usize) -> String {
        " ".repeat(num)
    }
}

impl writer::Writer {
    pub fn new(root: serde_json::Value) -> Self {
        Self { indent: 0, root }
    }

    pub fn write(&mut self) -> String {
        return self.write_value(self.root.clone());
    }

    pub fn write_value(&mut self, value: serde_json::Value) -> String {
        let mut result = String::new();
        match value {
            serde_json::Value::Object(obj) => {
                result.push_str(format!("@{{\n").as_str());
                self.indent += 4;

                for key in obj.keys() {
                    let value = self.write_value(obj[key].clone());
                    result.push_str(
                        format!("{}{} = {}\n", writer::padding(self.indent), key, value).as_str(),
                    );
                }
                self.indent -= 4;
                result.push_str(format!("{}}}\n", writer::padding(self.indent)).as_str());
            }
            serde_json::Value::Array(arr) => {
                result.push_str(format!("@(\n").as_str());
                self.indent += 4;

                for value in arr {
                    result.push_str(
                        format!(
                            "{}{}\n",
                            writer::padding(self.indent),
                            self.write_value(value)
                        )
                        .as_str(),
                    );
                }
                self.indent -= 4;
                result.push_str(format!("{})\n", writer::padding(self.indent)).as_str());
            }

            serde_json::Value::String(s) => result = format!("\"{}\"", s),
            serde_json::Value::Number(n) => result = format!("{}", n),
            serde_json::Value::Bool(b) => {
                result = match b {
                    true => "$true".to_string(),
                    false => "$false".to_string(),
                }
            }

            v => {
                panic!("Unsupported value type: {:?}", v)
            }
        }

        return result;
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input path
    #[clap(short, long)]
    input: PathBuf,

    /// Output path
    #[clap(short, long)]
    output: PathBuf,

    /// Mode (json2ps or ps2json)
    #[clap(short, long)]
    mode: String,
}

impl Args {
    fn check(&self) {
        if self.mode != "json2ps" && self.mode != "ps2json" {
            panic!("Invalid mode: {} allowed: json2ps, ps2json", self.mode);
        }
    }
}

fn read_file(path: PathBuf) -> String {
    let mut file = File::open(path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    contents
}

fn write_file(path: PathBuf, contents: String) {
    let mut file = File::create(path).expect("Unable to create file");
    file.write_all(contents.as_bytes())
        .expect("Unable to write file");
}

fn convert_ps2json(source: String) -> String {
    let mut scanner = lexer::Scanner::new(source);
    scanner.scan_tokens();

    let mut parser = parser::Parser::new(scanner.tokens);
    let res = parser.parse();

    serde_json::to_string_pretty(&res).unwrap()
}

fn convert_json2ps(source: String) -> String {
    let res: serde_json::Value = serde_json::from_str(&source).unwrap();
    let mut writer = writer::Writer::new(res);
    writer.write()
}

fn main() {
    let args = Args::parse();
    args.check();

    let source = read_file(args.input);

    let result = match args.mode.as_str() {
        "json2ps" => convert_json2ps(source),
        "ps2json" => convert_ps2json(source),
        _ => panic!("Invalid mode: {}", args.mode),
    };

    write_file(args.output, result);
}
