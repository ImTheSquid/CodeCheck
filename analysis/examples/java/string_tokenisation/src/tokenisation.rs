use std::fs::File;
use std::{fs, array};
use std::{path::PathBuf, borrow::Cow};
use std::io::{self, Write, Read};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use walkdir::WalkDir;
use std::hash::Hash;
mod r#match;
mod signature;
mod token;
mod codesight;


pub struct AssociatedStruct<'a, Ident, T> {
    /// The real owner of the AST
    pub owner: &'a Ident,
    /// The relative path of the source file the AST came from
    pub source: Cow<'a, str>,
    /// The inner item
    pub inner: T,
}

fn tokenisation<Ident: Hash + Clone + Send + Sync + 'static, S: AsRef<str>>(sources: &[AssociatedStruct<'_, Ident, S>], progress: Option<mpsc::Sender<usize>>) -> Result<Vec<Vec<f32>>, Box<dyn std::error::Error>> {

    let mut texts: Vec<String> = Vec::new();
    let mut len = 0;
    for source in sources {
        texts.push(source.inner.as_ref().to_string());
        len = len + 1;
    }

    let mut codesight = codesight::Codesight::new(texts);

    let mut signatures = Vec::new();
    for i in 0..len {
        signatures.push(codesight.get_signature(i).unwrap());
    }
    let mut matrix: Vec<Vec<f32>> = vec![vec![0.0; len]; len];


    for i in 0..len {
        for j in 0..len {
            if i != j {
                let match1 = codesight.greedy_string_tiling(&signatures[i], &signatures[j], 30);
                let match_percent = codesight.get_percent_match(&match1) / 100.00;
                
                // Report progress if a sender is provided
                if let Some(sender) = progress.as_ref() {
                    sender.send((i)).unwrap();
                }
                matrix[i][j] = match_percent;
            }
            if i == j {
                matrix[i][j] = 1.0;
            }  
        }
    }


    Ok(matrix)

}