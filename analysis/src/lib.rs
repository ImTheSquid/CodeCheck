use std::{fs, path::PathBuf};

use anyhow::Result;
use thiserror::Error;

mod c;
mod cpp;
mod gen;

/// Big-O runtime complexity
pub enum RuntimeComplexity {
    /// O(1)
    Constant,
    /// O(log(n))
    Logarithmic,
    /// O(n)
    Linear,
    /// O(nlog(n))
    SuperLinear,
    /// O(n^a)
    Polynomial(usize),
    /// O(2^n)
    Exponential,
    /// O(n!)
    Factorial,
}

/// Represents any tree for a specific language
pub trait SyntaxTree {
    /// Compares a tree with another tree of the specific language
    fn compare(&self, other: &Self) -> f64;

    /// Gets the worst runtime complexity within the tree
    fn worst_runtime_complexity(&self) -> RuntimeComplexity;

    /// Gets the runtime complexity of a single function, returns `Option::None` if not found
    fn runtime_complexity_of_fn<S: AsRef<str>>(&self, name: S) -> Option<RuntimeComplexity>;
}

/// Any errors that may occur when generating a parse tree
#[derive(Debug, Error)]
pub enum TreeParseError {
    #[error(transparent)]
    FileError(std::io::Error),
    #[error("Unknown language")]
    UnknownLanguage,
    #[error("Invalid node")]
    InvalidNode,
}

/// The language to be parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Java,
    C,
    Cpp,
    Python,
}

/// Attempts to parse a language and its tree from a string
pub fn generate_tree<S: AsRef<str>, T: SyntaxTree>(
    _input: S,
    _language: Language,
) -> Result<T, TreeParseError> {
    todo!()
}

/// Attempts to parse a language and its tree from a file
pub fn generate_tree_from_file<P: Into<PathBuf>, T: SyntaxTree>(
    input: P,
    language: Option<Language>,
) -> Result<T, TreeParseError> {
    let buf = input.into();
    generate_tree(
        fs::read_to_string(buf.clone()).map_err(TreeParseError::FileError)?,
        match language {
            Some(l) => l,
            None => guess_language_from_path(buf)?,
        },
    )
}

/// Attempts to guess the language of the file using a path
fn guess_language_from_path(path: PathBuf) -> Result<Language, TreeParseError> {
    match path
        .extension()
        .ok_or(TreeParseError::UnknownLanguage)?
        .to_str()
        .ok_or(TreeParseError::UnknownLanguage)?
    {
        "java" => Ok(Language::Java),
        "py" => Ok(Language::Python),
        "c" | "h" => Ok(Language::C),
        "cpp" | "cc" | "hh" | "cxx" | "hpp" | "hxx" => Ok(Language::Cpp),
        _ => Err(TreeParseError::UnknownLanguage),
    }
}

pub struct VisitorReturn<T>(Result<T, TreeParseError>);

impl<T> Default for VisitorReturn<T> {
    fn default() -> Self {
        Self(Err(TreeParseError::InvalidNode))
    }
}
