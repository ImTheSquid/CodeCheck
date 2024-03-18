#![feature(async_iterator)]
#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::{borrow::Cow, path::PathBuf};

use crate::c::{CTree, CTreeItem};
use antlr_rust::errors::ANTLRError;
use anyhow::Result;
use java::{JavaTree, JavaTreeItem};
use nalgebra::{DMatrix, Dyn, VecStorage};
use rayon::prelude::*;
use std::sync::mpsc;
use syntree::Empty;
use thiserror::Error;

mod c;
mod cpp;
mod java;
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
    type Item: PartialEq;
    fn symbol_tree(self) -> Result<syntree::Tree<Self::Item, Empty, usize>, TreeParseError>;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Java,
    C,
    Cpp,
    Python,
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

pub fn detect_plagiarism_in_sources<Ident: PartialEq + Clone + Send + Sync + 'static, S: AsRef<str> + Send>(
    sources: Vec<AssociatedStruct<'_, Ident, S>>,
    language: Option<Language>,
    progress: Option<mpsc::Sender<()>>,
) -> Result<DMatrix<f64>> {
    if sources.is_empty() {
        return Ok(DMatrix::from_data(VecStorage::new(
            Dyn(0),
            Dyn(0),
            Vec::new(),
        )));
    }

    let language = match language {
        None => guess_language_from_path(PathBuf::from(sources[0].source.as_ref()))?,
        Some(l) => l,
    };

    Ok(match language {
        Language::Java => TreeCompare::comparison_matrix(
            convert_sources_to_trees::<Ident, S, JavaTree, JavaTreeItem>(sources).into_iter().filter_map(Result::ok).collect(),
            progress,
        ),
        Language::C => TreeCompare::comparison_matrix(
            convert_sources_to_trees::<Ident, S, CTree, CTreeItem>(sources).into_iter().filter_map(Result::ok).collect(),
            progress,
        ),
        Language::Cpp => TreeCompare::comparison_matrix(
            convert_sources_to_trees::<Ident, S, CTree, CTreeItem>(sources).into_iter().filter_map(Result::ok).collect(),
            progress,
        ),
        Language::Python => todo!(),
    })
}

fn convert_sources_to_trees<'a, 'b, Ident: ToOwned + Sync + Send, S, T, I: Send>(
    sources: Vec<AssociatedStruct<'b, Ident, S>>,
) -> Vec<Result<AssociatedStruct<'b, Ident, Tree<I>>, TreeParseError>>
where
    S: AsRef<str> + Send + 'a,
    T: TryFrom<String, Error = TreeParseError> + SyntaxTree<Item = I>,
{
    // let mut out = Vec::with_capacity(sources.len());
    sources.into_par_iter().map(|source| {
        let inner_value = source.inner.as_ref().to_owned(); // Clone or convert as needed
        match T::try_from(inner_value) {
            Ok(t) => match t.symbol_tree() {
                Ok(st) => Ok(AssociatedStruct {
                    owner: source.owner.clone(),
                    source: source.source.clone(),
                    inner: st,
                }),
                Err(e) => Err(e),
            },
            Err(e) => Err(e),
        }
    }).collect::<Vec<_>>()
    // for source in sources {
    //     let inner_value = source.inner.as_ref().to_owned(); // Clone or convert as needed
    //     match T::try_from(inner_value) {
    //         Ok(t) => match t.symbol_tree() {
    //             Ok(st) => out.push(AssociatedStruct {
    //                 owner: source.owner.clone(),
    //                 source: source.source.clone(),
    //                 inner: st,
    //             }),
    //             Err(e) => return Err(e),
    //         },
    //         Err(e) => return Err(e),
    //     }
    // }
    // out
}

type Tree<TreeItem> = syntree::Tree<TreeItem, Empty, usize>;
type Node<'a, TreeItem> = syntree::Node<'a, TreeItem, Empty, usize>;

#[derive(Debug, Clone)]
pub struct AssociatedStruct<'a, Ident, T> {
    /// The real owner of the AST
    pub owner: Arc<Ident>,
    /// The relative path of the source file the AST came from
    pub source: Cow<'a, str>,
    /// The inner item
    pub inner: T,
}

#[derive(Debug, Clone)]
pub struct DepthAwareTree<TreeNode> {
    tree: Tree<TreeNode>,
    depth: usize,
}

impl<TreeNode> Deref for DepthAwareTree<TreeNode> {
    type Target = Tree<TreeNode>;

    fn deref(&self) -> &Self::Target {
        &self.tree
    }
}

impl<TreeNode, Ident> AssociatedStruct<'_, Ident, DepthAwareTree<TreeNode>> {
    fn first(&self) -> Option<AssociatedStruct<'_, Ident, Node<'_, TreeNode>>> {
        self.inner.tree.first().map(|n| AssociatedStruct {
            owner: self.owner.clone(),
            source: Cow::Borrowed(self.source.as_ref()),
            inner: n,
        })
    }

    // fn as_node(&self) -> AssociatedStruct<Ident, Node<'_, TreeNode>> {
    //     AssociatedStruct { owner: self.owner.clone(), source: self.source.clone(), inner: self.inner.first().unwrap() }
    // }
}

impl<Ident, T> Deref for AssociatedStruct<'_, Ident, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

const LAMBDA_TREE: f64 = 0.1;

pub struct TreeCompare<'a, Ident, TreeItem> {
    trees: Vec<AssociatedStruct<'a, Ident, DepthAwareTree<TreeItem>>>,
}

macro_rules! tree_depth {
    ($n:ident) => {
        $n.walk().with_depths().par_bridge().map(|(depth, _)| depth).max().unwrap_or(0)
    };
}

impl<Ident: PartialEq + Sync + Send + Clone, TreeItem: Sync + Send + PartialEq>
    TreeCompare<'_, Ident, TreeItem>
{
    pub fn comparison_matrix(
        trees: Vec<AssociatedStruct<'_, Ident, Tree<TreeItem>>>,
        progress: Option<mpsc::Sender<()>>,
    ) -> DMatrix<f64> {
        let trees = trees.into_par_iter().map(|t| {
            let tr = &t.inner;
            let depth = tree_depth!(tr);
            AssociatedStruct {
                owner: t.owner,
                source: t.source,
                inner: DepthAwareTree { tree: t.inner, depth }
            }
        }).collect::<Vec<_>>();
        let num_trees = trees.len();
        let comp = Arc::new(TreeCompare { trees });
        // let mat = Arc::new(RwLock::new(Option::Some(DMatrix::from_data(VecStorage::new(Dyn(comp.trees.len()), Dyn(comp.trees.len()), vec![1.0; comp.trees.len().pow(2)])))));

        let res: Vec<f64> = (0..num_trees)
            .into_par_iter()
            .map_with((comp.clone(), progress), move |bundle, i| {
                vec![0.0; i + 1]
                    .into_par_iter()
                    .chain((i + 1..num_trees).into_par_iter().map_with(
                        &*bundle,
                        |(comp, progress), j| {
                            let res = if comp.trees[i].owner != comp.trees[j].owner {
                                comp.k_prime(&comp.trees[i], &comp.trees[j])
                            } else {
                                -1.0
                            };
                            if let Some(progress) = progress {
                                progress.send(()).unwrap();
                            }

                            res
                        },
                    ))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        let mat = DMatrix::from_data(VecStorage::new(Dyn(num_trees), Dyn(num_trees), res));

        let mut mat = mat.clone() + mat.transpose();

        for i in 0..num_trees {
            mat[(i, i)] = 1.0;
        }

        mat
    }

    /// Cosine similarity
    fn k_prime(
        &self,
        a: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        b: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        let numerator = self.k(a, b);
        let denom_a = self.k(a, a);
        let denom_b = self.k(b, b);
        numerator / (denom_a * denom_b).sqrt()
    }

    fn k(
        &self,
        a: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        b: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        if !a.first().is_some_and(|n| n.has_children())
            || !b.first().is_some_and(|n| n.has_children())
        {
            return 0.0;
        }

        let a_subtrees = self.subtrees(&a.first().unwrap());
        let b_subtrees = self.subtrees(&b.first().unwrap());
        let b_subtrees = &b_subtrees;

        a_subtrees
            .into_par_iter()
            .map(|e_a| {
                b_subtrees
                    .into_par_iter()
                    .map(|e_b| self.c(&e_a, a, e_b, b))
                    .reduce(|| 0.0, |id, x| id + x)
            })
            .reduce(|| 0.0, |id, x| id + x)
    }

    fn subtrees<'a, 'b>(
        &self,
        tree: &AssociatedStruct<'b, Ident, Node<'a, TreeItem>>,
    ) -> Vec<AssociatedStruct<'b, Ident, Node<'a, TreeItem>>> {
        tree.inner
            .walk()
            .par_bridge()
            .map(|n| AssociatedStruct {
                owner: tree.owner.clone(),
                source: tree.source.clone(),
                inner: n,
            })
            .collect::<Vec<_>>()
    }

    fn c(
        &self,
        a: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>,
        a_full: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        b: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>,
        b_full: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        // Terminal nodes have no useful information and the parents were already compared in the else block, so just return 0
        if a.inner != b.inner || (!a.has_children() && !b.has_children()) {
            0.0
        } else {
            let product = (0..self.ns(a)).into_par_iter();
            let max_fn = (0..self.ns(b)).into_par_iter();

            LAMBDA_TREE
                * product
                    .map(|i| {
                        let c_max = max_fn
                            .clone()
                            .map(|j| {
                                let st_s1_i = AssociatedStruct {
                                    owner: a.owner.clone(),
                                    source: a.source.clone(),
                                    inner: a.children().nth(i).unwrap(),
                                };

                                let st_s2_j = AssociatedStruct {
                                    owner: b.owner.clone(),
                                    source: b.source.clone(),
                                    inner: b.children().nth(j).unwrap(),
                                };

                                self.c(&st_s1_i, a_full, &st_s2_j, b_full)
                            })
                            .reduce(|| 0.0, |id, x| id.max(x));

                        1.0 + c_max
                    })
                    .reduce(|| 1.0, |id, x| id * x)
                * self.w_st(a, a_full)
                * self.w_st(b, b_full)
        }
    }

    fn w_st(
        &self,
        subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>,
        tree: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        self.tf(subtree, tree) * self.idf(subtree)
    }

    fn cnt(
        &self,
        subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>,
        tree: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        self.subtree_appearances_in_tree(subtree, tree, false) as f64
            / self.n(&tree.first().unwrap()) as f64
    }

    fn n(&self, tree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>) -> usize {
        tree.walk().par_bridge().count()
    }

    fn idf(&self, subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>) -> f64 {
        (1.0 + self.trees.len() as f64 / self.trees_contain_s(subtree) as f64).log2()
    }

    fn trees_contain_s(&self, subtree: &Node<'_, TreeItem>) -> usize {
        self.trees.par_iter().filter(|tree| self.subtree_appearances_in_tree(subtree, tree, true) > 0).count()
    }

    /// TODO: Rayon optimize
    fn subtree_appearances_in_tree(
        &self,
        subtree: &Node<'_, TreeItem>,
        tree: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        find_first_only: bool,
    ) -> usize {
        if tree.depth < tree_depth!(subtree) {
            return 0;
        }

        let mut appearances = 0_usize;

        for node in tree.walk() {
            if subtree.value() == node.value() {
                let mut are_equal = true;
                // This is a really shitty way of doing it that I know has bugs, but it should work for now
                let mut node_iter = node.walk();
                let mut subtree_iter = subtree.walk();
                if (&mut node_iter).zip(&mut subtree_iter).par_bridge().any(|(s_node, t_node)| s_node.value() != t_node.value()) {
                    are_equal = false;
                }
                // while let Some((s_node, t_node)) = (&mut node_iter).zip(&mut subtree_iter).next() {
                //     if s_node.value() != t_node.value() {
                //         are_equal = false;
                //         break;
                //     }
                // }

                // If the subtree hasn't had all of its elements consumed, the trees must be different
                if subtree_iter.next().is_some() {
                    continue;
                }

                if are_equal {
                    if find_first_only {
                        return 1;
                    }
                    appearances += 1;
                }
            }
        }

        appearances
    }

    fn ns(&self, subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>) -> usize {
        subtree.children().count()
    }

    fn tf(
        &self,
        subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>,
        tree: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        self.cnt(subtree, tree) / self.n(&tree.first().unwrap()) as f64
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
            $lang::try_from($code.to_owned()).unwrap();
        }
    };
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{detect_plagiarism_in_sources, AssociatedStruct, Language};
    use nalgebra::matrix;
    use std::sync::Arc;

    #[test]
    fn compare_same() {
        let a = r#"#include"stdio.h"
int main()
{
int M,N;
scanf("%d",&M);
scanf("%d",&N);
if (M%N==0)
printf("YES\n");
else
printf("NO\n");
return 0;
}
"#;

        let store = vec![
            AssociatedStruct {
                owner: Arc::new(1234),
                source: std::borrow::Cow::Borrowed("a.c"),
                inner: a.to_string(),
            },
            AssociatedStruct {
                owner: Arc::new(5678),
                source: Cow::Borrowed("b.c"),
                inner: a.to_string(),
            },
        ];

        let res =
            detect_plagiarism_in_sources::<usize, String>(store, Some(Language::C), None).unwrap();
        assert_eq!(
            res,
            matrix![
                1.0, 1.0;
                1.0, 1.0;
            ]
        );
    }

    #[test]
    fn example_test() {
        let a = r#"#include"stdio.h"
int main()
{
int M,N;
scanf("%d",&M);
scanf("%d",&N);
if (M%N==0)
printf("YES\n");
else
printf("NO\n");
return 0;
}
"#;

        let b = r#"#include<stdio.h>
int main()
{int a,b;
scanf("%d",&a);
scanf("%d",&b);
if (a%b==0)
printf("YES");
else
printf("NO");




}
"#;

        let store = vec![
            AssociatedStruct {
                owner: Arc::new(1234),
                source: Cow::Borrowed("a.c"),
                inner: a.to_string(),
            },
            AssociatedStruct {
                owner: Arc::new(5678),
                source: Cow::Borrowed("b.c"),
                inner: b.to_string(),
            },
        ];

        let res =
            detect_plagiarism_in_sources::<usize, String>(store, Some(Language::C), None).unwrap();
        println!("{}", res);
        assert_ne!(res[(0, 1)], 1.0);
        assert_ne!(res[(0, 1)], 0.0);
        assert_ne!(res[(1, 0)], 1.0);
        assert_ne!(res[(1, 0)], 0.0);
    }

    #[test]
    fn slight_difference_test() {
        let a = r#"#include"stdio.h"
int main()
{
int M = 0;
int N = 4;
scanf("%d",&M);
scanf("%d",&N);
if (M%N!=0)
printf("YES\n");
else
printf("NO\n");
return 3;
}
"#;

        let b = r#"#include<stdio.h>
int main()
{int a,b;
scanf("%d",&a);
scanf("%d",&b);
if (a%b==0)
printf("YES");
else
printf("NO");




}
"#;

        let store = vec![
            AssociatedStruct {
                owner: Arc::new(1234),
                source: Cow::Borrowed("a.c"),
                inner: a.to_string(),
            },
            AssociatedStruct {
                owner: Arc::new(5678),
                source: Cow::Borrowed("b.c"),
                inner: b.to_string(),
            },
        ];

        let res =
            detect_plagiarism_in_sources::<usize, String>(store, Some(Language::C), None).unwrap();
        assert_ne!(res[(0, 1)], 1.0);
        assert_ne!(res[(0, 1)], 0.0);
        assert_ne!(res[(1, 0)], 1.0);
        assert_ne!(res[(1, 0)], 0.0);
    }
}
