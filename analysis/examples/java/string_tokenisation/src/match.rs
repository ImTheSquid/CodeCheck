use crate::{token::Token, signature::Signature};

pub struct Match {
    code1: Vec<i32>,
    code2: Vec<i32>,
    count: Vec<i32>,
    checked: Vec<bool>,
    tokens1: Vec<Token>,
    tokens2: Vec<Token>,
    threshold: i32,
    sig1: Signature,
    sig2: Signature,
}

impl Match {
    pub fn new(sig1: Signature, sig2: Signature, tokens1: Vec<Token>, tokens2: Vec<Token>, threshold: i32) -> Match {
        Match {
            code1: Vec::new(),
            code2: Vec::new(),
            count: Vec::new(),
            checked: Vec::new(),
            tokens1,
            tokens2,
            threshold,
            sig1,
            sig2,
        }
    }

    pub fn add(&mut self, i: i32, j: i32, k: i32) {
        self.code1.push(i);
        self.code2.push(j);
        self.count.push(k);
        self.checked.push(false);
    }

    pub fn get_size(&self) -> usize {
        self.code1.len()
    }

    pub fn get_threshold(&self) -> i32 {
        self.threshold
    }

    pub fn get_data(&self) -> Vec<[i32; 3]> {
        let mut data = Vec::new();
        for i in 0..self.code1.len() {
            data.push([self.code1[i], self.code2[i], self.count[i]]);
        }
        data
    }

    //not sure about this. Not sure about Integer class in Rust
    pub fn get_integer_data(&self) -> Vec<[i32; 3]> {
        let mut data = Vec::new();
        let mut j = 0;
        for i in 0..self.code1.len() {
            if self.checked[i] {
                data.push([self.code1[i], self.code2[i], self.count[i]]);
                j += 1;
            }
        }
        data
    }

    //not sure about this as well. Don't think its being used    
    pub fn get_object_data(&self) -> Vec<[i32; 5]> {
        let mut data = Vec::new();
        let mut j = 0;
        for i in 0..self.code1.len() {
            if self.checked[i] {
                data.push([j + 1, self.code1[i], self.code2[i], self.count[i], 0]);
                j += 1;
            }
        }
        data
    }

    pub fn count_checked(&self) -> usize {
        let mut total = 0;
        for checked in &self.checked {
            if *checked {
                total += 1;
            }
        }
        total
    }

    //figure out how to use Object. How to use vector with different types
    /*fn get_token_data(&self, select: i32) -> Option<Vec<Vec<Box<dyn std::any::Any>>>> {
        let mut data: Option<Vec<Vec<Box<dyn std::any::Any>>>> = None;
    
        match select {
            1 => {
                let mut tokens1_data: Vec<Vec<Box<dyn std::any::Any>>>;
                tokens1_data = Vec::with_capacity(self.tokens1.len());
    
                for (i, token) in self.tokens1.iter().enumerate() {
                    tokens1_data.push(vec![
                        Box::new(i),
                        Box::new(token.get_token()),
                        Box::new(token.get_word()),
                        Box::new(token.get_line()),
                        Box::new(token.get_mark()),
                    ]);
                }
    
                data = Some(tokens1_data);
            }
            2 => {
                let mut tokens2_data: Vec<Vec<Box<dyn std::any::Any>>>;
    
                tokens2_data = Vec::with_capacity(self.tokens2.len());
    
                for (i, token) in self.tokens2.iter().enumerate() {
                    tokens2_data.push(vec![
                        Box::new(i),
                        Box::new(token.get_token()),
                        Box::new(token.get_word()),
                        Box::new(token.get_line()),
                        Box::new(token.get_mark()),
                    ]);
                }
    
                data = Some(tokens2_data);
            }
            _ => {}
        }
    
        data
    }*/
    

    pub fn sort(&mut self, field: i32) {
        let len = self.count.len();
        for i in 0..len {
            for j in (i + 1)..len {
                match field {
                    1 => {
                        if self.code1[j] > self.code1[i] {
                            self.switch_position(i, j);
                        }
                    }
                    2 => {
                        if self.code2[j] > self.code2[i] {
                            self.switch_position(i, j);
                        }
                    }
                    3 => {
                        if self.count[j] > self.count[i] {
                            self.switch_position(i, j);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    fn switch_position(&mut self, i: usize, j: usize) {
        let a = self.code1[i];
        let b = self.code2[i];
        let c = self.count[i];
        self.code1[i] = self.code1[j];
        self.code2[i] = self.code2[j];
        self.count[i] = self.count[j];
        self.code1[j] = a;
        self.code2[j] = b;
        self.count[j] = c;
    }

    pub fn mark(&mut self) {
        for i in 0..self.get_size() {
            let a = self.code1[i] as usize;
            let b = self.code2[i] as usize;
            let count = self.count[i] as usize;
            let mut ini = 0;
            while ini < count
                && !self.tokens1[a + ini].get_mark()
                && !self.tokens2[b + ini].get_mark()
            {
                ini += 1;
            }
            if ini == count {
                for ini in 0..count {
                    self.tokens1[a + ini].mark();
                    self.tokens2[b + ini].mark();
                }
                self.checked[i] = true;
            }
        }
    }

    pub fn get_tokens_length(&self, id: i32) -> usize {
        match id {
            1 => self.tokens1.len(),
            2 => self.tokens2.len(),
            _ => 0,
        }
    }

    pub fn get_signature(&self, num: i32) -> Option<&Signature> {
        match num {
            1 => Some(&self.sig1),
            2 => Some(&self.sig2),
            _ => None,
        }
    }
}

