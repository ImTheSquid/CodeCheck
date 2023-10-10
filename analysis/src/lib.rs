use std::{fs, path::PathBuf};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use antlr_rust::errors::ANTLRError;
use anyhow::Result;
use downcast_rs::{Downcast, impl_downcast};
use nalgebra::DMatrix;
use thiserror::Error;
use crate::c::CTree;
use crate::cpp::CppTree;

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
pub trait SyntaxTree: Downcast {
    /// Compares a tree with another tree of the specific language
    fn compare(&self, other: &Box<dyn SyntaxTree>) -> f64;

    /// Gets the worst runtime complexity within the tree
    fn worst_runtime_complexity(&self) -> RuntimeComplexity;

    /// Gets the runtime complexity of a single function, returns `Option::None` if not found
    fn runtime_complexity_of_fn(&self, name: &str) -> Option<RuntimeComplexity>;
}

impl_downcast!(SyntaxTree);

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
) -> Result<Box<dyn SyntaxTree>, TreeParseError> {
    Ok(match language {
        Language::Java => todo!(),
        Language::C => Box::new(CTree::try_from(input.as_ref())?),
        Language::Cpp => Box::new(CppTree::try_from(input.as_ref())?),
        Language::Python => todo!(),
    })
}

/// Attempts to parse a language and its tree from a file
pub fn generate_tree_from_file<P: Into<PathBuf>>(
    input: P,
    language: Option<Language>,
) -> Result<Box<dyn SyntaxTree>, TreeParseError> {
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

type Tree<TreeItem> = syntree::Tree<TreeItem, usize, usize>;
type Node<'a, TreeItem> = syntree::Node<'a, TreeItem, usize, usize>;

struct AssociatedTree<'a, Ident, TreeItem> {
    /// The real owner of the AST
    owner: &'a Ident,
    /// The relative path of the source file the AST came from
    source: &'a str,
    /// The AST
    tree: &'a Tree<TreeItem>,
}

struct TreeCompare<'a, Ident: Hash, TreeItem> {
    trees: &'a [AssociatedTree<'a, Ident, TreeItem>],
    subtree_cache: HashMap<&'a str, Vec<Node<'a, TreeItem>>>,
}
const LAMBDA_TREE: f64 = 0.1;

impl<'a, Ident: Hash, TreeItem> TreeCompare<'a, Ident, TreeItem> {
    pub async fn comparison_matrix(trees: &'a [AssociatedTree<'a, Ident, TreeItem>]) -> DMatrix<f64> {
        let comp = TreeCompare { trees, subtree_cache: Default::default() };
        todo!()
    }

    fn ns(&mut self, subtree: &AssociatedTree<'a, Ident, TreeItem>) -> usize {
        self.subtrees(subtree).len()
    }

    /// This is a heavy operation, so cache as much as possible
    fn subtrees(&mut self, tree: &AssociatedTree<'a, Ident, TreeItem>) -> &[Node<'a, TreeItem>] {
        let mut hasher = DefaultHasher::default();
        tree.owner.hash(&mut hasher);
        let ident = format!("{}{}", hasher.finish(), tree.source);

        if let Some(cached) = self.subtree_cache.get(&ident) {
            return cached.as_slice();
        }

        let subtrees = tree.tree.children().into_iter().filter(|n| n.has_children()).collect::<Vec<_>>();
        self.subtree_cache.insert(&ident, subtrees);

        self.subtree_cache.get(ident).unwrap().as_slice()
    }

    fn cnt(&mut self, subtree: &Tree<TreeItem>, tree: &Tree<TreeItem>) -> f64 {
        todo!()
    }

    fn n(&mut self, tree: &AssociatedTree<'a, Ident, TreeItem>) -> usize {
        self.subtrees(tree).len()
    }

    fn w_st(&mut self, subtree: &Tree<TreeItem>, tree: &Tree<TreeItem>) -> f64 {
        self.tf(subtree, tree) * self.idf(subtree)
    }

    fn tf(&mut self, subtree: &Tree<TreeItem>, tree: &Tree<TreeItem>) -> f64 {
        self.cnt(subtree, tree) / self.n(tree) as f64
    }

    fn trees_contain_s(&mut self, subtree: &Tree<TreeItem>) -> usize {
        todo!()
    }

    fn idf(&mut self, subtree: &Tree<TreeItem>) -> f64 {
        (1.0 + self.trees.len() as f64 / self.trees_contain_s(subtree) as f64).log2()
    }

    fn k(&mut self, a: &Tree<TreeItem>, b: &Tree<TreeItem>) -> f64 {
        self.subtrees(a)
            .into_iter()
            .fold(0.0, |acc, e_a| acc + self.subtrees(b).into_iter().fold(0.0, |acc , e_b| acc + self.c(&e_a, a,&e_b, b)))
    }

    fn c(&mut self, a: &Tree<TreeItem>, a_full: &Tree<TreeItem>, b: &Tree<TreeItem>, b_full: &Tree<TreeItem>) -> f64 {
        if a.first() != b.first() {
            0.0
        } else if a.first().is_some_and(|n| !n.has_children()) && b.first().is_some_and(|n| !n.has_children()) {
            LAMBDA_TREE * self.w_st(a, a_full) * self.w_st(b, b_full)
        } else {
            let product = (1.0..=self.ns(a) as f64).into_iter();
            let max_iter = (1.0..=self.ns(b) as f64).into_iter();

            LAMBDA_TREE * product.fold(1.0, |acc, i| acc * (1 + max_iter.fold(0.0, |acc, j| acc.max(self.c(a, a_full, b, b_full))))) * self.w_st(a, a_full) * self.w_st(b, b_full)
        }
    }

    /// Cosine similarity
    fn k_prime(&mut self, a: &Tree<TreeItem>, b: &Tree<TreeItem>) -> f64 {
        self.k(a, b) / (self.k(a, a) * self.k(b, b)).sqrt()
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
