pub struct Token {
    token: String,
    word: String,
    line: i32,
    mark: bool,
}

impl Token {
    pub fn new(token: &str, word: &str, line: i32) -> Token {
        Token {
            token: token.to_string(),
            word: word.to_string(),
            line,
            mark: false,
        }
    }

    pub fn get_data(&self) -> [String; 4] {
        [self.token.clone(), self.word.clone(), self.line.to_string(), self.mark.to_string()]
    }

    pub fn get_line(&self) -> i32 {
        self.line
    }

    pub fn get_mark(&self) -> bool {
        self.mark
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }

    pub fn get_word(&self) -> &str {
        &self.word
    }

    pub fn mark(&mut self) {
        self.mark = true;
    }

    pub fn unmark(&mut self) {
        self.mark = false;
    }

    pub fn copy(&self) -> Token {
        let mut copy = Token::new(&self.token, &self.word, self.line);
        if self.mark {
            copy.mark();
        }
        copy
    }
}
