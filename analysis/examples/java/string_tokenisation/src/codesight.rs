use std::fs::{File, self};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::vec::Vec;
use std::collections::VecDeque;

use crate::r#match::Match;
use crate::signature::Signature;

pub struct Codesight {
    texts: Vec<String>,
    signatures: Vec<Signature>,
}

impl Codesight {
    pub fn new(texts: Vec<String>) -> Codesight {
        let mut signatures = Vec::new();
        for text in &texts {
            let signature = Signature::new(text.to_string());
            signatures.push(signature);
        }


        Codesight {
            texts,
            signatures,
        }

    }

    /*fn get_source_code(&self, index: usize) -> String {
        let mut source_code = String::new();
        if let Ok(file) = self.files[index].try_clone() {
            let reader = BufReader::new(file);
            let mut i = 0;
            for line in reader.lines() {
                if let Ok(line) = line {
                    i += 1;
                    source_code.push_str(&format!("{}   {}\n", i, line));
                }
            }
        }

        source_code
    }*/

    pub fn get_file_name(file_path: &str) -> Option<&str> {
        let path = Path::new(file_path);
        path.file_name().and_then(|name| name.to_str())
    }


    fn get_files_name(directory_path: &str) -> Vec<String> {
        let dir_entries = match fs::read_dir(directory_path) {
            Ok(entries) => entries,
            Err(_) => return Vec::new(), // Return an empty vector in case of an error.
        };

        let mut names = Vec::new();

        for entry in dir_entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    names.push(name.to_string());
                }
            }
        }

        names
    }


    pub fn get_signature(&self, index: usize) -> Option<Signature> {
        if index < self.signatures.len() {
            //println!("signature: {}", self.signatures[0].copy().get_num_tokens());
            Some((self.signatures[index]).copy())
        } else {
            None
        }
    }

    //don't know how to get this to work
    /*fn get_signature_by_name(&self, name: &str) -> Option<Signature> {
        for signature in &self.signatures {
            if signature.get_file_name() == name {
                return Some(signature.copy());
            }
        }
        None
    }*/

    pub fn greedy_string_tiling(&self, sig1: &Signature, sig2: &Signature, threshold: i32) -> Match {
        let a = sig1.get_token_data();
        let b = sig2.get_token_data();
        let len_a = a.len();
        let len_b = b.len();
        let mut matches = Match::new(sig1.copy(), sig2.copy(), a, b, threshold);
        let mut max_match = threshold;

        let a = sig1.get_token_data();
        let b = sig2.get_token_data();

        for a_idx in 0..len_a {
            for b_idx in 0..len_b {
                let mut j = 0;
                if a_idx == b_idx {
                    j = 1;
                }
                while a_idx + j < sig1.get_num_tokens()
                    && b_idx + j < sig2.get_num_tokens()
                    && a[a_idx + j].get_token().eq(b[b_idx + j].get_token())
                    && !a[a_idx + j].get_mark()
                    && !b[b_idx + j].get_mark()
                {
                    j += 1;
                }
                if j > 0 && j >= max_match as usize {
                    matches.add(a_idx as i32, b_idx as i32, j as i32);
                }
            }
        }

        if matches.get_size() > 0 {
            matches.sort(3);
            matches.mark();
        }

        matches
    }

    pub fn get_percent_match(&self, match_data: &Match) -> f32 {
        let mut percent = 1.0;
        let mut coverage = 0.0;
        let total: f32;

        let copied_sequences = match_data.count_checked();
        let data = match_data.get_integer_data();

        for i in 0..copied_sequences {
            coverage += data[i][2] as f32;
        }

        total = match_data.get_tokens_length(1) as f32 + match_data.get_tokens_length(2) as f32;

        //println!("total: {}", total);

        percent = (2.0 * coverage) / total;

        percent * 100.0
    }

    fn compare_all(&self, threshold: i32) -> Vec<Vec<f32>> {
        let mut table = vec![vec![-1.0; self.signatures.len()]; self.signatures.len()];

        for i in 0..self.signatures.len() {
            for j in i..self.signatures.len() {
                if i == j {
                    table[i][j] = -1.0;
                } else {
                    let result = self.greedy_string_tiling(&self.signatures[i], &self.signatures[j], threshold);
                    table[i][j] = self.round(self.get_percent_match(&result), 2);
                    table[j][i] = table[i][j];
                }
            }
        }

        table
    }

    fn get_titles(&self) -> Vec<String> {
        (0..self.signatures.len()).map(|i| i.to_string()).collect()
    }

    fn get_num_signatures(&self) -> usize {
        self.signatures.len()
    }

    fn round(&self, percent: f32, places: i32) -> f32 {
        let factor = 10_i32.pow(places as u32);
        let value = percent * factor as f32;
        let tmp = value.round() as i32;
        tmp as f32 / factor as f32
    }
}
