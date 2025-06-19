use antlr_rust::errors::ANTLRError;
use std::path::Path;
use strum::IntoEnumIterator;
use syntree::node::Event;

pub mod c;
pub mod cpp;
pub mod java;
pub mod python;
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
            Language::Python => python::PythonTreeItem::iter().count(),
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

/// Prune a tree by removing intermediate nodes with one child
pub fn prune_tree<T: Copy + std::fmt::Debug>(tree: &mut syntree::Tree<T, usize, usize>) {
    let mut blank: syntree::Builder<T, usize, usize> = Default::default();
    let mut single_child_stack = vec![];
    let mut is_entering_branch_node = false;
    let mut previous_node_was_terminal = false;
    for (event, node) in tree.walk_events() {
        let num_children = node.children().count();

        match event {
            Event::Down => {
                // When the tree advances down, see how many children the node has
                if num_children == 0 {
                    // No children, this is a token
                    if is_entering_branch_node {
                        // If we're entering a branch, that means our next event is NEXT, so push 0 onto the stack to prepare
                        single_child_stack.push(0);
                    } else {
                        // Not entering a branch, so increment the count of children for the parent node
                        *single_child_stack.last_mut().expect("some value on stack") += 1;
                    }
                    // Add the token to the tree
                    blank.token_with(node.value(), *node.span()).expect("token");
                } else {
                    // This node has children
                    if is_entering_branch_node {
                        // If we're entering a branch, that means our next event is either DOWN or NEXT
                        // Push 0 onto the stack to prepare for the next event
                        single_child_stack.push(0);

                        // Add the node to the tree
                        blank
                            .open_with(node.value(), *node.span())
                            .unwrap_or_else(|e| panic!("valid open on node {node:?}: {e}"));
                    } else {
                        // One child, so increment the count of children for the parent node
                        *single_child_stack.last_mut().expect("some value on stack") += 1;
                    }
                }
            }
            Event::Up => {
                if let Some(last) = single_child_stack.last_mut() {
                    if *last == 0 {
                        assert!(num_children > 1, "Should only be popping off the stack if has more than one child! Offender: {node:?}");
                        single_child_stack.pop();
                        // If the previous node was terminal, closing the node is a duplicate operation
                        // since the terminal node didn't open one of it's own
                        if !previous_node_was_terminal {
                            blank
                                .close()
                                .unwrap_or_else(|e| panic!("valid close on node {node:?}: {e}"));
                        }
                    } else {
                        *last -= 1;
                    }
                }
            }
            Event::Next => match single_child_stack.last() {
                Some(last) => {
                    // There's a value on the stack, so going to the next child of a multi-child node
                    assert_eq!(
                        *last, 0,
                        "invalid state: current stack item still has value >0"
                    );
                    // We're at the top, so close the previous node and open the next one (if it wasn't terminal)
                    if !previous_node_was_terminal {
                        blank
                            .close()
                            .unwrap_or_else(|e| panic!("valid next close on node {node:?}: {e}"));
                    }
                    if num_children > 0 {
                        blank
                            .open_with(node.value(), *node.span())
                            .unwrap_or_else(|e| panic!("valid next open on node {node:?}: {e}"));
                    } else {
                        blank.token_with(node.value(), *node.span()).expect("token");
                    }
                }
                None => {
                    // Starting state
                    single_child_stack.push(0);
                    blank
                        .open_with(node.value(), *node.span())
                        .unwrap_or_else(|e| panic!("valid next open on node {node:?}: {e}"));
                }
            },
        }

        if matches!(event, Event::Down | Event::Next) {
            is_entering_branch_node = num_children > 1;
        }

        previous_node_was_terminal = num_children == 0;
    }
    assert_eq!(
        single_child_stack.as_slice(),
        [0usize],
        "stack is not empty after pruning! {single_child_stack:?}"
    );

    // Close last open node
    blank.close().expect("final close");

    *tree = blank.build().expect("valid tree from already valid tree");
}
