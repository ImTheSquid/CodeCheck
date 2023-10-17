#![feature(async_iterator)]
#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::path::PathBuf;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::sync::Arc;

use antlr_rust::errors::ANTLRError;
use anyhow::Result;
use async_recursion::async_recursion;
use async_trait::async_trait;
use nalgebra::{DMatrix, Dyn, VecStorage};
use thiserror::Error;
use tokio::sync::RwLock;
use futures::StreamExt;
use crate::c::{CTree, CTreeItem};

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
    async fn read_files(&self) -> Result<Vec<AssociatedStruct<'_, Self::Ident, Self::S>>>;
}

pub async fn detect_plagiarism_in_sources<Ident: Hash + Clone + Send + Sync, S: AsRef<str>>(provider: &impl AssociatedFileProvider<Ident = Ident, S = S>, language: Option<Language>) -> Result<DMatrix<f64>> {
    let sources = provider.read_files().await?;

    if sources.is_empty() {
        return Ok(DMatrix::from_data(VecStorage::new(Dyn(0), Dyn(0), Vec::new())));
    }

    let language = match language {
        None => guess_language_from_path(PathBuf::from(sources[0].source))?,
        Some(l) => l,
    };

    Ok(match language {
        Language::Java => todo!(),
        Language::C => TreeCompare::comparison_matrix(convert_sources_to_trees::<'_, Ident, S, CTree, CTreeItem>(sources)?).await,
        Language::Cpp => todo!(),
        Language::Python => todo!(),
    })
}

fn convert_sources_to_trees<'a, Ident, S, T, I>(
    sources: Vec<AssociatedStruct<'a, Ident, S>>
) -> Result<Vec<AssociatedStruct<'a, Ident, Tree<I>>>, TreeParseError>
where
    S: AsRef<str> + 'a,
    T: TryFrom<String, Error = TreeParseError> + SyntaxTree<Item = I>,
{
    let mut out = Vec::with_capacity(sources.len());
    for source in sources {
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

#[derive(Debug)]
pub struct AssociatedStruct<'a, Ident, T> {
    /// The real owner of the AST
    owner: &'a Ident,
    /// The relative path of the source file the AST came from
    source: &'a str,
    /// The inner item
    inner: T,
}

impl<Ident, T: Clone> Clone for AssociatedStruct<'_, Ident, T> {
    fn clone(&self) -> Self {
        AssociatedStruct {
            owner: self.owner,
            source: self.source,
            inner: self.inner.clone(),
        }
    }
}

impl<'a, TreeNode, Ident> AssociatedStruct<'a, Ident, Tree<TreeNode>> {
    fn first(&self) -> Option<AssociatedStruct<'a, Ident, Node<'_, TreeNode>>> {
        self.inner.first().map(|n| AssociatedStruct {
            owner: self.owner,
            source: self.source,
            inner: n,
        })
    }
}

impl<'a, Ident, T> Deref for AssociatedStruct<'a, Ident, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug)]
struct Caches<'a, Ident: Hash, TreeItem> {
    subtree: HashMap<String, Vec<AssociatedStruct<'a, Ident, Node<'a, TreeItem>>>>
}

#[derive(Debug, Clone)]
pub struct TreeCompare<'a, Ident: Hash, TreeItem> {
    trees: Vec<AssociatedStruct<'a, Ident, Tree<TreeItem>>>,
    cache: Arc<RwLock<Caches<'a, Ident, TreeItem>>>,
}
const LAMBDA_TREE: f64 = 0.1;

impl<'a, Ident: Hash + Sync + Clone, TreeItem: PartialEq + Sync + Send + Clone + 'static> TreeCompare<'a, Ident, TreeItem> {
    pub async fn comparison_matrix(trees: Vec<AssociatedStruct<'a, Ident, Tree<TreeItem>>>) -> DMatrix<f64> {
        let comp = Arc::new(TreeCompare { trees, cache: Arc::new(RwLock::new(Caches { subtree: HashMap::new() })) });
        let mut mat = DMatrix::from_data(VecStorage::new(Dyn(comp.trees.len()), Dyn(comp.trees.len()), vec![1.0; comp.trees.len().pow(2)]));

        for i in 0..=comp.trees.len() - 1 {
            for j in i + 1..comp.trees.len() {
                mat[(i, j)] = comp.k_prime(&comp.trees[i], &comp.trees[j]).await;
            }
        }

        mat.lower_triangle()
    }

    async fn ns(&self, subtree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>) -> usize {
        subtree.children().count()
    }

    /// This is a heavy operation, so cache as much as possible
    async fn subtrees(&self, tree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>) -> Vec<AssociatedStruct<'a, Ident, Node<'a, TreeItem>>> {
        let mut hasher = DefaultHasher::default();
        tree.owner.hash(&mut hasher);
        let ident = format!("{}{}", hasher.finish(), tree.source);

        if let Some(cached) = self.cache.read().await.subtree.get(&ident) {
            return cached.to_vec();
        }

        let subtrees = tree.walk().filter(|n| n.has_children()).map(|n| AssociatedStruct {
            owner: tree.owner,
            source: tree.source,
            inner: n,
        }).collect::<Vec<_>>();
        self.cache.write().await.subtree.insert(ident.clone(), subtrees);

        self.cache.read().await.subtree.get(&ident).unwrap().to_vec()
    }

    async fn subtree_appearances_in_tree(&self, subtree: &Node<'a, TreeItem>, tree: &AssociatedStruct<'a, Ident, Tree<TreeItem>>) -> usize {
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

    async fn cnt(&self, subtree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>, tree: &'a AssociatedStruct<'a, Ident, syntree::Tree<TreeItem, usize, usize>>) -> f64 {
        self.subtree_appearances_in_tree(subtree, tree).await as f64 / self.n(&tree.first().unwrap()).await as f64
    }

    async fn n(&self, tree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>) -> usize {
        self.subtrees(tree).await.len()
    }

    async fn w_st(&self, subtree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>, tree: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>) -> f64 {
        self.tf(subtree, tree).await * self.idf(subtree).await
    }

    async fn tf(&self, subtree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>, tree: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>) -> f64 {
        self.cnt(subtree, tree).await / self.n(&tree.first().unwrap()).await as f64
    }

    async fn trees_contain_s(&self, subtree: &Node<'a, TreeItem>) -> usize {
        let mut count = 0_usize;
        for tree in &self.trees {
            if self.subtree_appearances_in_tree(subtree, tree).await > 0 {
                count += 1;
            }
        }

        count
    }

    async fn idf(&self, subtree: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>) -> f64 {
        (1.0 + self.trees.len() as f64 / self.trees_contain_s(subtree).await as f64).log2()
    }

    async fn k(&self, a: &'a AssociatedStruct<'a, Ident, syntree::Tree<TreeItem, usize, usize>>, b: &'a AssociatedStruct<'a, Ident, syntree::Tree<TreeItem, usize, usize>>) -> f64 {
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
    async fn c(&self, a: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>, a_full: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>, b: &AssociatedStruct<'a, Ident, Node<'a, TreeItem>>, b_full: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>) -> f64 {
        if a.first() != b.first() {
            0.0
        } else if a.first().is_some_and(|n| !n.has_children()) && b.first().is_some_and(|n| !n.has_children()) {
            LAMBDA_TREE * self.w_st(a, a_full).await * self.w_st(b, b_full).await
        } else {
            let product = futures::stream::iter(1..self.ns(a).await);

            LAMBDA_TREE * product.fold(1.0, async move |acc, i| acc * (1.0 + futures::stream::iter(1..self.ns(b).await).fold(0.0_f64, async move |acc, j| acc.max(self.c(&AssociatedStruct {
                owner: a.owner,
                source: a.source,
                inner: a.children().nth(i).unwrap(),
            }, a_full, &AssociatedStruct {
                owner: b.owner,
                source: b.source,
                inner: b.children().nth(j).unwrap(),
            }, b_full).await)).await)).await * self.w_st(a, a_full).await * self.w_st(b, b_full).await
        }
    }

    /// Cosine similarity
    async fn k_prime(&self, a: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>, b: &'a AssociatedStruct<'a, Ident, Tree<TreeItem>>) -> f64 {
        self.k(a, b).await / (self.k(a, a).await * self.k(b, b).await).sqrt()
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
    use nalgebra::{DMatrix, Dyn, VecStorage};
    use crate::{AssociatedFileProvider, AssociatedStruct, detect_plagiarism_in_sources, Language, TreeCompare};

    struct PhonyProvider<'a> {
        store: Vec<AssociatedStruct<'a, usize, String>>,
    }

    #[async_trait]
    impl AssociatedFileProvider for PhonyProvider<'_> {
        type Ident = usize;
        type S = String;
        async fn read_files(&self) -> anyhow::Result<Vec<AssociatedStruct<'_, Self::Ident, Self::S>>> {
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
                owner: &1234,
                source: "a.c",
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: &5678,
                source: "b.c",
                inner: a.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_eq!(res[(0, 0)], 1.0);
        assert_eq!(res[(0, 1)], 0.0);
        assert_eq!(res[(1, 1)], 1.0);
        assert_eq!(res[(1, 0)], 1.0);
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
                owner: &1234,
                source: "a.c",
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: &5678,
                source: "b.c",
                inner: b.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_eq!(res[(0, 0)], 1.0);
        assert_eq!(res[(0, 1)], 0.0);
        assert_eq!(res[(1, 1)], 1.0);
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
                owner: &1234,
                source: "a.c",
                inner: a.to_string(),
            }, AssociatedStruct {
                owner: &5678,
                source: "b.c",
                inner: b.to_string(),
            }]
        };

        let res = detect_plagiarism_in_sources::<usize, String>(&store, Some(Language::C)).await.unwrap();
        assert_eq!(res[(0, 0)], 1.0);
        assert_eq!(res[(0, 1)], 0.0);
        assert_eq!(res[(1, 1)], 1.0);
        assert_ne!(res[(1, 0)], 1.0);
        assert_ne!(res[(1, 0)], 0.0);
    }
}
