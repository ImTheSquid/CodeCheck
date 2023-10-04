use std::{fs, path::PathBuf};

use antlr_rust::errors::ANTLRError;
use anyhow::Result;
use thiserror::Error;
use crate::c::CTree;

mod c;
mod cpp;
#[macro_use]
mod gen;

#[macro_export]
macro_rules! visitor_result {
    ($x:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => return VisitorReturn(Err(TreeParseError::from(e))),
        }
    };
}

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
    fn runtime_complexity_of_fn(&self, name: &str) -> Option<RuntimeComplexity>;
}

/// Any errors that may occur when generating a parse tree
#[derive(Debug, Error)]
pub enum TreeParseError {
    #[error(transparent)]
    FileError(#[from] std::io::Error),
    #[error("Unknown language")]
    UnknownLanguage,
    #[error("Invalid node")]
    InvalidNode,
    #[error("Missing node")]
    MissingNode,
    #[error(transparent)]
    TreeError(#[from] syntree::Error),
    #[error(transparent)]
    AntlrError(#[from] Box<ANTLRError>),
    #[error("This is a placeholder error for a temporary tree result, something else went wrong")]
    PlaceholderError,
    #[error("The input was empty")]
    Empty,
}

impl From<ANTLRError> for TreeParseError {
    fn from(value: ANTLRError) -> Self {
        Self::from(Box::new(value))
    }
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
pub fn generate_tree<'a, S: AsRef<str>>(
    input: S,
    language: Language,
) -> Result<impl SyntaxTree, TreeParseError> {
    match language {
        Language::Java => todo!(),
        Language::C => Ok(CTree::try_from(input.as_ref())?),
        Language::Cpp => todo!(),
        Language::Python => todo!(),
    }
}

/// Attempts to parse a language and its tree from a file
pub fn generate_tree_from_file<P: Into<PathBuf>>(
    input: P,
    language: Option<Language>,
) -> Result<impl SyntaxTree, TreeParseError> {
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

#[derive(Debug)]
pub struct VisitorReturn<T>(Result<T, TreeParseError>);

impl<T> Default for VisitorReturn<T> {
    fn default() -> Self {
        Self(Err(TreeParseError::PlaceholderError))
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! test_parse {
    ($name: ident, $lang: ident, $code: expr) => {
        #[test]
        fn $name() {
            $lang::try_from($code).unwrap();
        }
    };
}
