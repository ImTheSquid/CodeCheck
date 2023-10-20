#![feature(async_iterator)]
#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::path::PathBuf;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

use antlr_rust::errors::ANTLRError;
use anyhow::Result;
use async_recursion::async_recursion;
use async_trait::async_trait;
use java::{JavaTree, JavaTreeItem};
use nalgebra::{DMatrix, Dyn, VecStorage};
use thiserror::Error;
use tokio::sync::RwLock;
use futures::{StreamExt, future::try_join_all};
use crate::c::{CTree, CTreeItem};

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
    fn symbol_tree(self) -> Result<syntree::Tree<Self::Item, usize, usize>, TreeParseError>;
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

#[async_trait]
pub trait AssociatedFileProvider {
    type Ident: Hash;
    type S: AsRef<str>;
    async fn read_files(&self) -> anyhow::Result<Vec<AssociatedStruct<Self::Ident, Self::S>>>;
}

pub async fn detect_plagiarism_in_sources<Ident: Hash + Clone + Send + Sync + 'static, S: AsRef<str>>(provider: &impl AssociatedFileProvider<Ident = Ident, S = S>, language: Option<Language>) -> Result<DMatrix<f64>> {
    let sources = provider.read_files().await?;

    if sources.is_empty() {
        return Ok(DMatrix::from_data(VecStorage::new(Dyn(0), Dyn(0), Vec::new())));
    }

    let language = match language {
        None => guess_language_from_path(PathBuf::from(sources[0].source.as_str()))?,
        Some(l) => l,
    };

    Ok(match language {
        Language::Java => TreeCompare::comparison_matrix(convert_sources_to_trees::<'_, Ident, S, JavaTree, JavaTreeItem>(sources)?).await?,
        Language::C => TreeCompare::comparison_matrix(convert_sources_to_trees::<'_, Ident, S, CTree, CTreeItem>(sources)?).await?,
        Language::Cpp => TreeCompare::comparison_matrix(convert_sources_to_trees::<'_, Ident, S, CTree, CTreeItem>(sources)?).await?,
        Language::Python => todo!(),
    })
}

fn convert_sources_to_trees<'a, Ident, S, T, I>(
    sources: Vec<AssociatedStruct<Ident, S>>
) -> Result<Vec<AssociatedStruct<Ident, Tree<I>>>, TreeParseError>
where
    S: AsRef<str> + 'a,
    T: TryFrom<String, Error = TreeParseError> + SyntaxTree<Item = I>,
{
    let mut out = Vec::with_capacity(sources.len());
    for source in sources {
        println!("{}", source.source);
        let inner_value = source.inner.as_ref().to_owned(); // Clone or convert as needed
        match T::try_from(inner_value) {
            Ok(t) => match t.symbol_tree() {
                Ok(st) => out.push(AssociatedStruct {
                    owner: source.owner,
                    source: source.source,
                    inner: st,
                }),
                Err(e) => return Err(e),
            }
            Err(e) => return Err(e),
        }
    }
    Ok(out)
}

type Tree<TreeItem> = syntree::Tree<TreeItem, usize, usize>;
type Node<'a, TreeItem> = syntree::Node<'a, TreeItem, usize, usize>;

#[derive(Debug, Clone)]
pub struct AssociatedStruct<Ident, T> {
    /// The real owner of the AST
    pub owner: Ident,
    /// The relative path of the source file the AST came from
    pub source: String,
    /// The inner item
    pub inner: T,
}

impl<TreeNode, Ident: Clone> AssociatedStruct<Ident, Tree<TreeNode>> {
    fn first(&self) -> Option<AssociatedStruct<Ident, Node<'_, TreeNode>>> {
        self.inner.first().map(|n| AssociatedStruct {
            owner: self.owner.clone(),
            source: self.source.clone(),
            inner: n,
        })
    }
}

impl<Ident, T> Deref for AssociatedStruct<Ident, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct TreeCompare<Ident: Hash, TreeItem> {
    trees: Vec<AssociatedStruct<Ident, Tree<TreeItem>>>,
}
const LAMBDA_TREE: f64 = 0.1;

impl<Ident: Hash + Sync + Clone + Send + 'static, TreeItem: PartialEq + Sync + Send + Clone + 'static> TreeCompare<Ident, TreeItem> {
    pub async fn comparison_matrix(trees: Vec<AssociatedStruct<Ident, Tree<TreeItem>>>) -> Result<DMatrix<f64>> {
        let comp = Arc::new(TreeCompare { trees });
        let mat = Arc::new(RwLock::new(Option::Some(DMatrix::from_data(VecStorage::new(Dyn(comp.trees.len()), Dyn(comp.trees.len()), vec![1.0; comp.trees.len().pow(2)])))));

        let num_trees = comp.trees.len() as f64;
        let mut futs = Vec::with_capacity((num_trees * num_trees.log10()) as usize);

        for i in 0..=comp.trees.len() - 1 {
            for j in i + 1..comp.trees.len() {
                let comp = comp.clone();
                let mat = mat.clone();
                futs.push(tokio::spawn(async move {
                    println!("COMPARING ON {i}:{j}");
                    let res = comp.k_prime(&comp.trees[i], &comp.trees[j]).await;
                    
                    let mut mat = mat.write().await;
                    let mat = mat.as_mut().unwrap();
                    mat[(i, j)] = res;
                    mat[(j, i)] = res;
                }));
            }
        }

        try_join_all(futs.into_iter()).await?;

        let mut mat = mat.write().await;
        Ok(mat.take().unwrap())
    }

    async fn ns(&self, subtree: &AssociatedStruct<Ident, Node<'_, TreeItem>>) -> usize {
        subtree.children().count()
    }

    /// This is a heavy operation, so cache as much as possible
    async fn subtrees<'a>(&self, tree: &AssociatedStruct<Ident, Node<'a, TreeItem>>) -> Vec<AssociatedStruct<Ident, Node<'a, TreeItem>>> {
        tree.walk().filter(|n| n.has_children()).map(|n| AssociatedStruct {
            owner: tree.owner.to_owned(),
            source: tree.source.to_owned(),
            inner: n,
        }).collect::<Vec<_>>()
    }

    async fn subtree_appearances_in_tree(&self, subtree: &Node<'_, TreeItem>, tree: &AssociatedStruct<Ident, Tree<TreeItem>>) -> usize {
        let mut appearances = 0_usize;

        for node in tree.walk() {
            if subtree.value() == node.value() {
                let mut are_equal = true;
                // This is a really shitty way of doing it that I know has bugs, but it should work for now
                let mut node_iter = node.walk();
                let mut subtree_iter = subtree.walk();
                while let Some((s_node, t_node)) = (&mut node_iter).zip(&mut subtree_iter).next() {
                    if s_node.value() != t_node.value() {
                        are_equal = false;
                        break
                    }
                }

                // If the zip had an unequal number of elements, the trees can't be the same
                if node_iter.next().is_some() || subtree_iter.next().is_some() {
                    continue
                }

                if are_equal {
                    appearances += 1;
                }
            }
        }

        appearances
    }

    async fn cnt(&self, subtree: &AssociatedStruct<Ident, Node<'_, TreeItem>>, tree: &AssociatedStruct<Ident, syntree::Tree<TreeItem, usize, usize>>) -> f64 {
        self.subtree_appearances_in_tree(subtree, tree).await as f64 / self.n(&tree.first().unwrap()).await as f64
    }

    async fn n(&self, tree: &AssociatedStruct<Ident, Node<'_, TreeItem>>) -> usize {
        self.subtrees(tree).await.len()
    }

    async fn w_st(&self, subtree: &AssociatedStruct<Ident, Node<'_, TreeItem>>, tree: &AssociatedStruct<Ident, Tree<TreeItem>>) -> f64 {
        self.tf(subtree, tree).await * self.idf(subtree).await
    }

    async fn tf(&self, subtree: &AssociatedStruct<Ident, Node<'_, TreeItem>>, tree: &AssociatedStruct<Ident, Tree<TreeItem>>) -> f64 {
        self.cnt(subtree, tree).await / self.n(&tree.first().unwrap()).await as f64
    }

    async fn trees_contain_s(&self, subtree: &Node<'_, TreeItem>) -> usize {
        let mut count = 0_usize;
        for tree in &self.trees {
            if self.subtree_appearances_in_tree(subtree, tree).await > 0 {
                count += 1;
            }
        }

        count
    }

    async fn idf(&self, subtree: &AssociatedStruct<Ident, Node<'_, TreeItem>>) -> f64 {
        (1.0 + self.trees.len() as f64 / self.trees_contain_s(subtree).await as f64).log2()
    }

    async fn k(&self, a: &AssociatedStruct<Ident, syntree::Tree<TreeItem, usize, usize>>, b: &AssociatedStruct<Ident, syntree::Tree<TreeItem, usize, usize>>) -> f64 {
        if !a.first().is_some_and(|n| n.has_children()) || !b.first().is_some_and(|n| n.has_children()) {
            return 0.0;
        }

        futures::stream::iter(self.subtrees(&a.first().unwrap()).await)
            .fold(0.0, async move |acc, e_a| {
                acc + futures::stream::iter(self.subtrees(&b.first().unwrap()).await).zip(futures::stream::repeat_with(|| e_a.clone())).fold(0.0, async move |acc, (e_b, e_a) | {
                    acc + self.c(&e_a, a, &e_b, b).await
                }).await
            }).await
    }

    #[async_recursion]
    async fn c(&self, a: &AssociatedStruct<Ident, Node<'_, TreeItem>>, a_full: &AssociatedStruct<Ident, Tree<TreeItem>>, b: &AssociatedStruct<Ident, Node<'_, TreeItem>>, b_full: &AssociatedStruct<Ident, Tree<TreeItem>>) -> f64 {
        if a.first() != b.first() {
            0.0
        } else if a.first().is_some_and(|n| !n.has_children()) && b.first().is_some_and(|n| !n.has_children()) {
            LAMBDA_TREE * self.w_st(a, a_full).await * self.w_st(b, b_full).await
        } else {
            let product = futures::stream::iter(1..self.ns(a).await);

            LAMBDA_TREE * product.fold(1.0, async move |acc, i| acc * (1.0 + futures::stream::iter(1..self.ns(b).await).fold(0.0_f64, async move |acc, j| acc.max(self.c(&AssociatedStruct {
                owner: a.owner.clone(),
                source: a.source.clone(),
                inner: a.children().nth(i).unwrap(),
            }, a_full, &AssociatedStruct {
                owner: b.owner.clone(),
                source: b.source.clone(),
                inner: b.children().nth(j).unwrap(),
            }, b_full).await)).await)).await * self.w_st(a, a_full).await * self.w_st(b, b_full).await
        }
    }

    /// Cosine similarity
    async fn k_prime(&self, a: &AssociatedStruct<Ident, Tree<TreeItem>>, b: &AssociatedStruct<Ident, Tree<TreeItem>>) -> f64 {
        let numerator = self.k(a, b).await;
        let denom_a = self.k(a, a).await;
        let denom_b = self.k(b, b).await;

        numerator / (denom_a * denom_b).sqrt()
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
    use async_trait::async_trait;
    use nalgebra::matrix;
    use crate::{AssociatedFileProvider, AssociatedStruct, detect_plagiarism_in_sources, Language};

    struct PhonyProvider {
        store: Vec<AssociatedStruct<usize, String>>,
    }

    #[async_trait]
    impl AssociatedFileProvider for PhonyProvider {
        type Ident = usize;
        type S = String;
        async fn read_files(&self) -> anyhow::Result<Vec<AssociatedStruct<Self::Ident, Self::S>>> {
            Ok(self.store.clone())
        }
    }

    #[tokio::test]
    async fn compare_same() {
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

        let store = PhonyProvider {
            store: vec![AssociatedStruct {
                owner: 1234,
                source: "a.c".to_string(),
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: 5678,
                source: "b.c".to_string(),
                inner: a.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_eq!(res, matrix![
            1.0, 1.0;
            1.0, 1.0;
        ]);
    }

    #[tokio::test]
    async fn example_test() {
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

        let store = PhonyProvider {
            store: vec![AssociatedStruct {
                owner: 1234,
                source: "a.c".to_string(),
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: 5678,
                source: "b.c".to_string(),
                inner: b.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_ne!(res[(0, 1)], 1.0);
        assert_ne!(res[(0, 1)], 0.0);
        assert_ne!(res[(1, 0)], 1.0);
        assert_ne!(res[(1, 0)], 0.0);
    }

        #[tokio::test]
    async fn slight_difference_test() {
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

        let store = PhonyProvider {
            store: vec![AssociatedStruct {
                owner: 1234,
                source: "a.c".to_string(),
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: 5678,
                source: "b.c".to_string(),
                inner: b.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_ne!(res[(0, 1)], 1.0);
        assert_ne!(res[(0, 1)], 0.0);
        assert_ne!(res[(1, 0)], 1.0);
        assert_ne!(res[(1, 0)], 0.0);
    }
}
