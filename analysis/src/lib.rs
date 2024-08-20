#![feature(async_iterator)]
#![feature(async_closure)]
#![feature(iterator_try_collect)]

use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use std::{borrow::Cow, path::PathBuf};

use anyhow::Result;
use ast::c::{CTree, CTreeItem};
use ast::cpp::{CppTree, CppTreeItem};
use ast::java::{JavaTree, JavaTreeItem};
use ast::{guess_language_from_path, Language, SyntaxTree, TreeParseError};
use nalgebra::{DMatrix, Dyn, VecStorage};
use rayon::prelude::*;
use std::sync::mpsc;
use syntree::Empty;
use thiserror::Error;

use fxhash::FxHashMap;

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

// pub fn detect_plagiarism_in_sources<
//     Ident: PartialEq + Clone + Send + Sync + 'static,
//     S: AsRef<str> + Send,
// >(
//     sources: Vec<AssociatedStruct<'_, Ident, S>>,
//     language: Option<Language>,
//     progress: Option<mpsc::Sender<()>>,
// ) -> Result<DMatrix<f64>> {
//     if sources.is_empty() {
//         return Ok(DMatrix::from_data(VecStorage::new(
//             Dyn(0),
//             Dyn(0),
//             Vec::new(),
//         )));
//     }

//     let language = match language {
//         None => guess_language_from_path(PathBuf::from(sources[0].source.as_ref()))?,
//         Some(l) => l,
//     };

//     Ok(match language {
//         Language::Java => TreeCompare::comparison_matrix(
//             convert_sources_to_trees::<Ident, S, JavaTree, JavaTreeItem>(sources)
//                 .into_iter()
//                 .filter_map(Result::ok)
//                 .collect(),
//             progress,
//         ),
//         Language::C => TreeCompare::comparison_matrix(
//             convert_sources_to_trees::<Ident, S, CTree, CTreeItem>(sources)
//                 .into_iter()
//                 .filter_map(Result::ok)
//                 .collect(),
//             progress,
//         ),
//         Language::Cpp => TreeCompare::comparison_matrix(
//             convert_sources_to_trees::<Ident, S, CppTree, CppTreeItem>(sources)
//                 .into_iter()
//                 .filter_map(Result::ok)
//                 .collect(),
//             progress,
//         ),
//         Language::Python => todo!(),
//     })
// }

// fn convert_sources_to_trees<'a, 'b, Ident: ToOwned + Sync + Send, S, T, I: Send>(
//     sources: Vec<AssociatedStruct<'b, Ident, S>>,
// ) -> Vec<Result<AssociatedStruct<'b, Ident, Tree<I>>, TreeParseError>>
// where
//     S: AsRef<str> + Send + 'a,
//     T: TryFrom<String, Error = TreeParseError> + SyntaxTree<Item = I>,
// {
//     // let mut out = Vec::with_capacity(sources.len());
//     sources
//         .into_par_iter()
//         .map(|source| {
//             let inner_value = source.inner.as_ref().to_owned(); // Clone or convert as needed
//             match T::try_from(inner_value) {
//                 Ok(t) => match t.symbol_tree() {
//                     Ok(st) => Ok(AssociatedStruct {
//                         owner: source.owner.clone(),
//                         source: source.source.clone(),
//                         inner: st,
//                     }),
//                     Err(e) => Err(e),
//                 },
//                 Err(e) => Err(e),
//             }
//         })
//         .collect::<Vec<_>>()
//     // for source in sources {
//     //     let inner_value = source.inner.as_ref().to_owned(); // Clone or convert as needed
//     //     match T::try_from(inner_value) {
//     //         Ok(t) => match t.symbol_tree() {
//     //             Ok(st) => out.push(AssociatedStruct {
//     //                 owner: source.owner.clone(),
//     //                 source: source.source.clone(),
//     //                 inner: st,
//     //             }),
//     //             Err(e) => return Err(e),
//     //         },
//     //         Err(e) => return Err(e),
//     //     }
//     // }
//     // out
// }

type Tree<TreeItem> = syntree::Tree<UniqueItem<TreeItem>, usize, usize>;
type Node<'a, TreeItem> = syntree::Node<'a, UniqueItem<TreeItem>, usize, usize>;

#[derive(Debug, Clone, Copy)]
pub struct UniqueItem<Item> {
    pub item: Item,
    pub id: uuid::Uuid,
}

impl<Item> PartialEq for UniqueItem<Item>
where
    Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.item == other.item
    }
}

impl<Node> UniqueItem<Node> {
    pub fn new(item: Node) -> Self {
        Self {
            item,
            id: uuid::Uuid::new_v4(),
        }
    }
}

impl<Node> Deref for UniqueItem<Node> {
    type Target = Node;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

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
        self.inner
            .tree
            .first()
            .map(|n: Node<'_, TreeNode>| AssociatedStruct {
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

const LAMBDA_TREE: f64 = 0.2;

pub struct TreeCompare<'a, Ident, TreeItem> {
    trees: Vec<AssociatedStruct<'a, Ident, DepthAwareTree<TreeItem>>>,
    tree_size_cache: std::sync::RwLock<FxHashMap<uuid::Uuid, usize>>,
    subtree_depth_cache: RwLock<FxHashMap<uuid::Uuid, usize>>,
    // Cache for trees_contain_s
    subtree_containment_cache: RwLock<FxHashMap<uuid::Uuid, usize>>,
    // Cache for the k(a, a) calls, where both trees are the same
    k_self_cache: RwLock<FxHashMap<String, f64>>,
}

macro_rules! tree_depth {
    ($n:ident) => {
        $n.walk()
            .with_depths()
            .par_bridge()
            .map(|(depth, _)| depth)
            .max()
            .unwrap_or(0)
    };
}

impl<'tree, Ident: PartialEq + Sync + Send + Clone, TreeItem: Sync + Send + PartialEq>
    TreeCompare<'tree, Ident, TreeItem>
{
    pub fn comparison_matrix(
        trees: Vec<AssociatedStruct<'tree, Ident, Tree<TreeItem>>>,
        progress: Option<mpsc::Sender<()>>,
    ) -> DMatrix<f64> {
        let trees = trees
            .into_par_iter()
            .map(|t| {
                let tr = &t.inner;
                let depth = tree_depth!(tr);
                AssociatedStruct {
                    owner: t.owner,
                    source: t.source,
                    inner: DepthAwareTree {
                        tree: t.inner,
                        depth,
                    },
                }
            })
            .collect::<Vec<_>>();
        let num_trees = trees.len();

        let comp = Arc::new(TreeCompare {
            trees,
            tree_size_cache: RwLock::new(FxHashMap::default()),
            subtree_depth_cache: RwLock::new(FxHashMap::default()),
            subtree_containment_cache: RwLock::new(FxHashMap::default()),
            k_self_cache: RwLock::new(FxHashMap::default()),
        });

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
                                println!(
                                    "Compare {} to {}",
                                    comp.trees[i].source, comp.trees[j].source
                                );
                                comp.k_prime(&comp.trees[i], &comp.trees[j])
                            } else {
                                -1.0
                            };
                            if let Some(progress) = progress {
                                progress.send(()).unwrap();
                            }

                            res.min(1.0)
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

    fn k_cache(&self, t: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>) -> f64 {
        if let Some(k) = self
            .k_self_cache
            .read()
            .expect("k cache read")
            .get(&t.source.to_string())
        {
            return *k;
        }
        let k = self.k(t, t);
        self.k_self_cache
            .write()
            .expect("k cache write")
            .insert(t.source.to_string(), k);
        k
    }

    /// Cosine similarity
    fn k_prime(
        &self,
        a: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        b: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
    ) -> f64 {
        let numerator = self.k(a, b);
        let denom_a = self.k_cache(a);
        let denom_b = self.k_cache(b);
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

        let a_first = a.first().unwrap();
        let b_first = b.first().unwrap();
        let a_subtrees = self.subtrees(&a_first);
        let b_subtrees = self.subtrees(&b_first);
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
        tree: &'b AssociatedStruct<'b, Ident, Node<'a, TreeItem>>,
    ) -> Vec<AssociatedStruct<'b, Ident, Node<'a, TreeItem>>> {
        tree.inner
            .walk()
            .par_bridge()
            .map(|n| AssociatedStruct {
                owner: tree.owner.clone(),
                source: Cow::Borrowed(tree.source.as_ref()),
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
                                    source: Cow::Borrowed(a.source.as_ref()),
                                    inner: a.children().nth(i).unwrap(),
                                };

                                let st_s2_j = AssociatedStruct {
                                    owner: b.owner.clone(),
                                    source: Cow::Borrowed(b.source.as_ref()),
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

    fn n(&self, node: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>) -> usize {
        if let Some(depth) = self
            .tree_size_cache
            .read()
            .expect("depth cache read")
            .get(&node.value().id)
        {
            return *depth;
        }
        let depth = node.walk().par_bridge().count();
        self.tree_size_cache
            .write()
            .expect("depth cache write")
            .insert(node.value().id, depth);
        depth
    }

    fn idf(&self, subtree: &AssociatedStruct<'_, Ident, Node<'_, TreeItem>>) -> f64 {
        (1.0 + self.trees.len() as f64 / self.trees_contain_s(subtree) as f64).log2()
    }

    fn trees_contain_s(&self, subtree: &Node<'_, TreeItem>) -> usize {
        if let Some(contain_count) = self
            .subtree_containment_cache
            .read()
            .expect("containment cache read")
            .get(&subtree.value().id)
        {
            return *contain_count;
        }

        let containment = self
            .trees
            .par_iter()
            .filter(|tree| self.subtree_appearances_in_tree(subtree, tree, true) > 0)
            .count();

        self.subtree_containment_cache
            .write()
            .expect("containment cache write")
            .insert(subtree.value().id, containment);

        containment
    }

    /// TODO: Rayon optimize
    fn subtree_appearances_in_tree(
        &self,
        subtree: &Node<'_, TreeItem>,
        tree: &AssociatedStruct<'_, Ident, DepthAwareTree<TreeItem>>,
        find_first_only: bool,
    ) -> usize {
        // This has to be written this way to not deadlock the RwLock
        let subtree_depth = 'std: {
            if let Some(depth) = self
                .subtree_depth_cache
                .read()
                .expect("st depth cache read")
                .get(&subtree.value().id)
            {
                break 'std *depth;
            }
            let depth = tree_depth!(subtree);
            self.subtree_depth_cache
                .write()
                .expect("st depth cache write")
                .insert(subtree.value().id, depth);
            depth
        };
        if tree.depth < subtree_depth {
            return 0;
        }

        let mut appearances = 0_usize;

        for (node_depth, node) in tree.walk().with_depths() {
            // Looking at each node in the tree to see if the subtree can fit
            if tree.depth - node_depth < subtree_depth {
                continue;
            }

            if **subtree.value() == **node.value() {
                let mut are_equal = true;
                // This is a really shitty way of doing it that I know has bugs, but it should work for now
                let mut node_iter = node.walk();
                let mut subtree_iter = subtree.walk();
                if (&mut node_iter)
                    .zip(&mut subtree_iter)
                    .par_bridge()
                    .any(|(s_node, t_node)| **s_node.value() != **t_node.value())
                {
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
