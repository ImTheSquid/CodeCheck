use std::fs::File;
use std::path::{PathBuf, Path};
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use regex::Regex;
use crate::token::Token;

pub struct Signature {
    text: String,
    total_lines: i32,
    lexical_stream: String,
    tokens: Vec<Token>,
}

static mut IN_MLC: bool   = false;

impl Signature {
    pub fn new(text: String) -> Self {
        let mut signature = Signature {
            text,
            total_lines: 0,
            lexical_stream: String::new(),
            tokens: Vec::new(),
        };

        //let reader = BufReader::new(&signature.file);
        let mut line_counter = 0;

        //signature.text = signature.remove_comments_string(signature.text.to_string());

        for line in signature.text.lines() {
            line_counter += 1;
            //let line = line.unwrap();
            let line = line.trim();
            let trimmed_line = signature.trim_comments(&line);
            let words = signature.separate_words(&trimmed_line);

            if !words.is_empty() {
                let lexical_stream = signature.get_lexical_stream(&words);
                let line_tokens: Vec<&str> = lexical_stream.split(' ').collect();

                for (i, word) in words.iter().enumerate() {
                    signature.tokens.push(Token::new(line_tokens[i], &word.to_string(), line_counter));
                    signature.lexical_stream.push(' ');
                    signature.lexical_stream.push_str(line_tokens[i]);
                }
            }
        }

        signature.total_lines = line_counter;
        signature.lexical_stream = signature.lexical_stream.trim().to_string();

        signature
    }

    pub fn trim_comments(&self, line: &str) -> String {
        if (line.contains("/*")) {
            unsafe {
                IN_MLC = true;
            }
        }
        if (line.contains("*/")) {
            unsafe {
                IN_MLC = false;
            }
            return String::new();
        }

        if (unsafe { IN_MLC }) {
            return String::new();
        }

        if line.len() > 1 && line.starts_with("//") {
            String::new()
        } else {
            line.to_string()
        }
    }

    fn remove_comments_string(&self, mut input: String) -> String {
        // https://stackoverflow.com/questions/54586730/how-to-remove-comments-denoted-by-and-in-a-string
        if input.contains("/*") && input.contains("*/") {
            let first_occurrence_open = input.find("/*").unwrap();
            let first_occurrence_trailing = input.find("*/").unwrap();
    
            input = Self::replace_at(first_occurrence_open, first_occurrence_trailing, input);
    
            if input.contains("/*") && input.contains("*/") {
                input = self.remove_comments_string(input);
            } else {
                return input;
            }
        }
    
        input
    }
    
    fn replace_at(mut start_index_incl: usize, mut end_index_incl: usize, data: String) -> String {
        // -2 because space and \, +2 because /\\*
        if (start_index_incl < 2) {
            start_index_incl = 2;
        }
        if (end_index_incl + 2 >= data.len()) {
            end_index_incl = data.len() - 3;
        }
        let replaced = format!(
            "{}{}",
            &data[..start_index_incl - 2],
            &data[end_index_incl + 2..]
        );
        replaced
    }
   

    pub fn separate_words(&self, line: &str) -> VecDeque<String> {
        let mut words = VecDeque::new();

        if !line.is_empty() {
            let mut line = line.to_string();
            let mut i = 0;

            while i < line.len() {
                if self.is_punctuation(&line[i..i + 1]) {
                    if i == 0 {
                        words.push_back(line[i..i + 1].trim().to_string());
                    } else {
                        words.push_back(line[0..i].to_string());
                        if !line[i..i + 1].trim().is_empty() {
                            words.push_back(line[i..i + 1].to_string());
                        }
                    }
                    line = line[i + 1..].trim().to_string();
                    i = 0;
                } else {
                    i += 1;
                }
            }

            if i == line.len() && !line.trim().is_empty() {
                words.push_back(line);
            }
        }

        words
    }

    pub fn get_lexical_stream(&self, words: &VecDeque<String>) -> String {
        let mut stream = String::new();
        let mut token;

        for word in words {
            token = word.to_string();
            if self.is_reserved_word(&token) {
                stream.push_str(&format!(" LITERAL_{}", token));
            } else if self.is_punctuation(&token) {
                match &token as &str {
                    "(" => stream.push_str(" LPAREN"),
                    ")" => stream.push_str(" RPAREN"),
                    "{" => stream.push_str(" LCURLY"),
                    "}" => stream.push_str(" RCURLY"),
                    "[" => stream.push_str(" LBRACK"),
                    "]" => stream.push_str(" RBRACK"),
                    "=" => stream.push_str(" ASSIGN"),
                    "<" => stream.push_str(" LT"),
                    ">" => stream.push_str(" BT"),
                    "+" => stream.push_str(" PLUS"),
                    "-" => stream.push_str(" MINUS"),
                    "*" => stream.push_str(" TIMES"),
                    "/" => stream.push_str(" DIV"),
                    "?" => stream.push_str(" QUEST"),
                    "!" => stream.push_str(" ADMIR"),
                    "," => stream.push_str(" COMMA"),
                    "." => stream.push_str(" DOT"),
                    ";" => stream.push_str(" SEMI"),
                    ":" => stream.push_str(" TWODOT"),
                    "@" => stream.push_str(" AT"),
                    "#" => stream.push_str(" SHARP"),
                    "%" => stream.push_str(" PERCENT"),
                    "\"" => stream.push_str(" QUOTE"),
                    "&" => stream.push_str(" AMPERS"),
                    "$" => stream.push_str(" DOLLAR"),
                    "\\" => stream.push_str(" BCKSLSH"),
                    "'" => stream.push_str(" SQUOTE"),
                    "~" => stream.push_str(" TILDE"),
                    "^" => stream.push_str(" CARET"),
                    "|" => stream.push_str(" VERBAR"),
                    _ => {}
                }
            } else if self.is_int(&token) {
                stream.push_str(" NUM_INT");
            } else if self.is_float(&token) {
                stream.push_str(" NUM_FLOAT");
            } else if self.is_double(&token) {
                stream.push_str(" NUM_DOUBLE");
            } else if self.is_long(&token) {
                stream.push_str(" NUM_LONG");
            } else {
                stream.push_str(" IDENT");
            }
        }

        stream.trim().to_string()
    }

    pub fn is_double(&self, word: &str) -> bool {
        word.parse::<f64>().is_ok()
    }

    pub fn is_int(&self, word: &str) -> bool {
        word.parse::<i32>().is_ok()
    }

    pub fn is_float(&self, word: &str) -> bool {
        word.parse::<f32>().is_ok()
    }

    pub fn is_long(&self, word: &str) -> bool {
        word.parse::<i64>().is_ok()
    }

    pub fn is_punctuation(&self, letter: &str) -> bool {
        match letter {
            " " | "\t" | "." | "," | "<" | ">" | "/" | "?" | ";" | ":" | "'" | "\"" | "~"
            | "[" | "]" | "{" | "}" | "\\" | "|" | "!" | "@" | "#" | "$" | "%" | "^" | "&" | "*"
            | "(" | ")" | "-" | "+" | "=" | "^" => true,
            _ => false,
        }
    }

    pub fn is_reserved_word(&self, word: &str) -> bool {
        match word {
            "asm" | "auto" | "bool"  | "break" | "case" | 
    "catch" | "char" | "class" | "const" | "continue" | "default" | "delete" | "do" | "double" | 
    "else" | "enum" | "extern" | "float" | "for" | "friend" | "goto" | "if" | "inline" |"int" | 
    "long" | "namespace" | "new" | "operator" | "private" | "protected" | "public" | 
    "register" | "return" | "short" | "signed" | "sizeof" | "static" | "struct" | "switch" | 
    "template" | "this" | "throw" | "try" | "typedef" | "union" | "unsigned" | "using" | 
    "virtual" | "void" | "volatile" | "while" => true,
            _ => false,
        }
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        for token in &self.tokens {
            tokens.push(token.copy());
        }
        tokens
    }

    pub fn copy(&self) -> Signature {
        let mut new = Signature::new(self.text.clone());
        new.total_lines = self.total_lines;
        new.lexical_stream = self.lexical_stream.clone();
        new.tokens = self.get_tokens();

        new
    }

    /*pub fn get_file(&self) -> File {
        self.file.try_clone().unwrap()
    }*/

    pub fn get_token_data(&self) -> Vec<Token> {
        let mut token_data = Vec::new();
    
        for token in self.tokens.iter() {
            token_data.push(token.copy());
        }
    
        token_data
    }
    

    //cannot figure out a way to get the file name from the file since
    // file object does not store the file path and file name
    /*pub fn get_file_name(&self) -> Option<&str> {
        self.file
    }*/


    pub fn mark(&mut self, index: usize) {
        if index < self.tokens.len() {
            self.tokens[index].mark();
        }
    }

    pub fn unmark(&mut self, index: usize) {
        if index < self.tokens.len() {
            self.tokens[index].unmark();
        }
    }

    pub fn get_num_tokens(&self) -> usize {
        self.tokens.len()
    }
}
