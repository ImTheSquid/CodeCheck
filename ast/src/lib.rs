use antlr_rust::errors::ANTLRError;
use std::path::Path;
use strum::IntoEnumIterator;

pub mod c;
pub mod cpp;
pub mod java;
#[macro_use]
pub mod gen;

#[macro_export]
macro_rules! visitor_result {
    ($x:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => return VisitorReturn(Err(TreeParseError::from(e))),
        }
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! test_parse {
    ($name: ident, $lang: ident, $code: expr) => {
        #[test]
        fn $name() {
            $lang::try_from($code.to_owned()).unwrap();
        }
    };
}

#[derive(Debug)]
pub struct VisitorReturn<T>(Result<T, TreeParseError>);

impl<T> Default for VisitorReturn<T> {
    fn default() -> Self {
        Self(Err(TreeParseError::PlaceholderError))
    }
}

/// Represents any tree for a specific language
pub trait SyntaxTree {
    type Item: PartialEq + Copy;
    fn symbol_tree(self) -> Result<syntree::Tree<Self::Item, usize, usize>, TreeParseError>;
}

/// Any errors that may occur when generating a parse tree
#[derive(Debug, thiserror::Error)]
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
    #[error("ANTLR Error: {0}")]
    AntlrError(String),
    #[error("This is a placeholder error for a temporary tree result, something else went wrong")]
    PlaceholderError,
    #[error("The input was empty")]
    Empty,
}

impl From<ANTLRError> for TreeParseError {
    fn from(value: ANTLRError) -> Self {
        TreeParseError::AntlrError(value.to_string())
    }
}

/// The language to be parsed
#[derive(Debug, Clone, Copy, PartialEq, Eq, strum::EnumIter)]
pub enum Language {
    C,
    Cpp,
    Java,
    Python,
}

impl Language {
    fn num_types(&self) -> usize {
        match self {
            Language::C => c::CTreeItem::iter().count(),
            Language::Cpp => cpp::CppTreeItem::iter().count(),
            Language::Java => java::JavaTreeItem::iter().count(),
            Language::Python => 0,
        }
    }

    /// Returns leading and trailing padding for the language
    pub fn padding(&self) -> (usize, usize) {
        let mut leading_sum = 0;
        let mut trailing_sum: usize = Language::iter().skip(1).map(|l| l.num_types()).sum();

        let langs = Language::iter().collect::<Vec<_>>();
        for (i, lang) in langs.iter().enumerate() {
            if *lang == *self {
                return (leading_sum, trailing_sum);
            }
            leading_sum += lang.num_types();
            // Safety: If trailing sum is greater than zero, it means there are more languages to process.
            if trailing_sum > 0 {
                trailing_sum -= langs[i + 1].num_types();
            }
        }
        unreachable!()
    }
}

/// Attempts to guess the language of the file using a path
pub fn guess_language_from_path(path: &Path) -> Result<Language, TreeParseError> {
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
