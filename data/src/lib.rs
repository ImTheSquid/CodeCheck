use std::{collections::HashMap, fmt::Display};

use ast::{Language, guess_language_from_extension};
use eyre::{Ok, Result, bail, eyre};
use fancy_regex::Regex;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};

const PROMPT: &str = include_str!("prompt.txt");
const COMPLEX_EXAMPLES: &str = "Implement a Persistent Segment Tree with Lazy Propagation, Suffix Automaton Construction and Applications, Minimum Cost Maximum Flow, Dynamic Connectivity with Link-Cut Trees, Heavy-Light Decomposition with LCA and Path Queries, Implement Alpha-Beta Pruning with Transposition Tables for Chess, Reinforcement Learning from Scratch, Backpropagation for a Deep Neural Network without Libraries, Bayesian Network Inference Engine, Design and Implement a Minimal Multithreaded Kernel, Virtual Memory Management Simulator, Custom File System on a Virtual Disk, Distributed Consensus with Paxos or Raft, Build a Scalable Key-Value Store (like Redis), TCP over UDP (Reliable Data Transfer Protocol), RSA Cryptosystem Implementation with Attacks, Zero-Knowledge Proof Protocol Simulator, Design a Sandbox using Seccomp and Linux Namespaces, Write a Recursive Descent Parser and Intermediate Code Generator, Type Inference for Lambda Calculus with Hindley-Milner, Symbolic Execution Engine, Implement Convex Hull Trick with Line Container, Fast Multipoint Polynomial Evaluation using NTT, 3D Computational Geometry: Mesh Boolean Operations";
const AVERAGE_EXAMPLES: &str = "Implement Dijkstra’s Algorithm, Depth-First Search and Breadth-First Search, Binary Search on a Sorted Array, Implement a Binary Search Tree with Insert/Delete, Find Lowest Common Ancestor in a Binary Tree, Implement Merge Sort and Quick Sort, Dynamic Programming: Longest Increasing Subsequence, Knapsack Problem (0/1), Detect Cycle in a Graph (DFS or Union-Find), Implement an LRU Cache, Two-Pointer Technique for Array Problems, Sliding Window Maximum, Design a HashMap from Scratch, Topological Sort of a Directed Acyclic Graph, Implement Trie with Insert/Search, Find All Anagrams in a String, Kadane’s Algorithm for Maximum Subarray Sum, Matrix Rotation (in-place), Implement a Queue using Stacks, Find All Paths Between Two Nodes in a Graph, Balanced Parentheses Checker using Stack, Floyd-Warshall All-Pairs Shortest Path, Binary Tree Level Order Traversal, Implement Min Heap and Max Heap";
const SIMPLE_EXAMPLES: &str = "Reverse a String, Check if a Number is Prime, Find Factorial of a Number, Fibonacci Sequence (Iterative and Recursive), Find the Maximum Element in an Array, Check if a String is a Palindrome, Count Vowels in a String, Sum of Elements in an Array, Find the Largest of Three Numbers, Linear Search in an Array, Print Multiplication Table, Check if a Number is Even or Odd, Swap Two Variables, Find GCD of Two Numbers, Print First N Natural Numbers, Convert Celsius to Fahrenheit, Count Words in a String, Check for Armstrong Number, Find the Length of a String, Simple Calculator (Add, Subtract, Multiply, Divide), Find the Number of Digits in an Integer, Reverse an Integer, Check for Leap Year, Generate Random Numbers, Sort an Array using Bubble Sort";

#[derive(Debug, PartialEq, Eq)]
pub struct PlagiarismEvent {
    pub file: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct GenerationOutput {
    pub pairs: HashMap<String, Vec<PlagiarismEvent>>,
    pub codes: Vec<(String, Language)>,
    pub topic: String,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ProblemComplexity {
    Simple,
    Average,
    Complex,
}

impl Display for ProblemComplexity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            ProblemComplexity::Complex => "very complex",
            ProblemComplexity::Simple => "simple",
            ProblemComplexity::Average => "average",
        };

        f.write_str(res)
    }
}

fn generate_prompt(complexity: ProblemComplexity, banned_topics: &[String]) -> String {
    let mut prompt = PROMPT
        .replace("{{topic_length}}", &format!("{complexity}"))
        .replace(
            "{{example_topics}}",
            match complexity {
                ProblemComplexity::Simple => SIMPLE_EXAMPLES,
                ProblemComplexity::Average => AVERAGE_EXAMPLES,
                ProblemComplexity::Complex => COMPLEX_EXAMPLES,
            },
        );

    prompt = format!("{prompt}\n{}", banned_topics.join("\n"));

    prompt
}

fn process_code(mut response: String) -> Result<GenerationOutput> {
    let topic_regex = Regex::new(r"<topic>(.+?)</topic>")?;
    let topic = topic_regex
        .captures(&response)?
        .ok_or(eyre!("no topic found!"))?[1]
        .to_string();

    response = topic_regex.replace_all(&response, "").to_string();

    let code_regex = Regex::new(r#"(?ms)```([^\n]*)\n(.*?)\n?```"#)?;
    let plagiarism_regex = Regex::new(r#"(?ms)\s*<plag (.+?)>(.*?)\s*</plag \1>"#)?;
    let mut cleaned_code: Vec<(String, Language)> = vec![];

    let mut plagiarism_by_id: HashMap<String, Vec<PlagiarismEvent>> = Default::default();

    for (code_i, m) in code_regex
        .captures_iter(&response)
        .filter_map(Result::ok)
        .enumerate()
    {
        let lang = guess_language_from_extension(&m[1])?;
        let code = m[2].to_string();
        let mut line_buffer = String::default();
        let mut found_open_plag_on_line_number: Option<usize> = None;
        let mut current_line_number = 1usize;
        for line in code.lines() {
            if line.contains("<plag ") {
                if let Some(ln) = found_open_plag_on_line_number {
                    eprintln!("Mismatched opening plagiarism tags detected. Ignoring first tag.");
                    line_buffer = line_buffer
                        .lines()
                        .enumerate()
                        .filter_map(|(i, l)| if i != ln - 1 { Some(l) } else { None })
                        .collect();
                    current_line_number -= 1;
                } else {
                    found_open_plag_on_line_number = Some(current_line_number);
                }
            }

            if plagiarism_regex.is_match(&line_buffer)? {
                // The current buffer has a full plagiarism match!
                let caps = plagiarism_regex
                    .captures_iter(&line_buffer)
                    .next()
                    .expect("capture from if statement")?;

                let id = caps[1].to_string();

                let plag = caps[2].to_string();
                let Some(start) = found_open_plag_on_line_number else {
                    bail!("no start when found full capture!");
                };
                let end = plag.lines().count() - 2 + start;
                line_buffer = plagiarism_regex.replace(&line_buffer, "$2").to_string();
                found_open_plag_on_line_number = None;
                // Start and end plagiarism tags removed
                current_line_number -= match end - start {
                    0 => 0,
                    1 => 1,
                    _ => 2,
                };

                let ev = PlagiarismEvent {
                    file: code_i,
                    start,
                    end,
                };
                match plagiarism_by_id.get_mut(&id) {
                    Some(v) => {
                        v.push(ev);
                    }
                    None => {
                        plagiarism_by_id.insert(id, vec![ev]);
                    }
                }
            }

            if line_buffer.is_empty() {
                line_buffer = line.to_string();
            } else {
                line_buffer = format!("{line_buffer}\n{line}");
            }
            current_line_number += 1;
        }

        cleaned_code.push((line_buffer, lang));
    }

    for (k, v) in plagiarism_by_id.iter() {
        if v.len() < 2 {
            bail!("unpaired plagiarism identifier {k}!");
        }
    }

    Ok(GenerationOutput {
        pairs: plagiarism_by_id,
        codes: cleaned_code,
        topic,
    })
}

pub async fn generate_code(
    ollama: &Ollama,
    model_name: String,
    banned_topics: &[String],
    content_length: ProblemComplexity,
) -> Result<GenerationOutput> {
    let response = ollama
        .generate(GenerationRequest::new(
            model_name,
            generate_prompt(content_length, banned_topics),
        ))
        .await?;

    process_code(response.response)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ast::Language;

    use crate::{PlagiarismEvent, process_code};

    const INPUT: &str = r#"<topic>Binary Search Tree</topic>
```java
public class BinaryTree {
    <plag tree_init>
    node = null;
    if (value != null) {
        node = new Node(value);
    }
    </plag tree_init>
    public void insert(int value) {
        <plag tree_insert>
        if (node == null) {
            node = new Node(value);
        } else {
            insertRec(node, value);
        }
        </plag tree_insert>
    }

    private void insertRec(Node current, int value) {
        if (value < current.value) {
            if (current.left == null) {
                current.left = new Node(value);
            } else {
                insertRec(current.left, value);
            }
        } else if (value > current.value) {
            if (current.right == null) {
                current.right = new Node(value);
            } else {
                insertRec(current.right, value);
            }
        }
    }
}
```

```java
public class AnotherTree {
    <plag tree_init>
    root = null;
    if (data != null) {
        root = new Node(data);
    }
    </plag tree_init>
    public void add(int data) {
        <plag tree_insert>
        if (root == null) {
            root = new Node(data);
        } else {
            insertRecursive(root, data);
        }
        </plag tree_insert>
    }

    private void insertRecursive(Node current, int data) {
        while (true) {
            if (data < current.value) {
                if (current.left == null) {
                    current.left = new Node(data);
                    break;
                } else {
                    current = current.left;
                }
            } else if (data > current.value) {
                if (current.right == null) {
                    current.right = new Node(data);
                    break;
                } else {
                    current = current.right;
                }
            }
        }
    }
}
```

```c
typedef struct Node {
    int value;
    struct Node* left;
    struct Node* right;
} Node;

void insert_node(Node** node, int value) {
    <plag tree_insert>
    if (*node == NULL) {
        *node = malloc(sizeof(Node));
        (*node)->value = value;
        (*node)->left = NULL;
        (*node)->right = NULL;
    } else {
        insert_rec(*node, value);
    }
    </plag tree_insert>
}

void insert_rec(Node* current, int value) {
    if (value < current->value) {
        if (current->left == NULL) {
            current->left = malloc(sizeof(Node));
            current->left->value = value;
            current->left->left = NULL;
            current->left->right = NULL;
        } else {
            insert_rec(current->left, value);
        }
    } else if (value > current->value) {
        if (current->right == NULL) {
            current->right = malloc(sizeof(Node));
            current->right->value = value;
            current->right->left = NULL;
            current->right->right = NULL;
        } else {
            insert_rec(current->right, value);
        }
    }
}
```

```python
class Node:
    def __init__(self, value):
        <plag tree_init>
        self.value = value
        self.left = None
        self.right = None
        </plag tree_init>

class BinaryTree:
    def __init__(self):
        self.root = None

    def insert(self, value):
        <plag tree_insert>
        if self.root is None:
            self.root = Node(value)
        else:
            self.insert_rec(self.root, value)
        </plag tree_insert>

    def insert_rec(self, current, value):
        if value < current.value:
            if current.left is None:
                current.left = Node(value)
            else:
                self.insert_rec(current.left, value)
        elif value > current.value:
            if current.right is None:
                current.right = Node(value)
            else:
                self.insert_rec(current.right, value)
```
"#;

    const CODES: [&str; 4] = [
        r#"public class BinaryTree {
    node = null;
    if (value != null) {
        node = new Node(value);
    }
    public void insert(int value) {
        if (node == null) {
            node = new Node(value);
        } else {
            insertRec(node, value);
        }
    }

    private void insertRec(Node current, int value) {
        if (value < current.value) {
            if (current.left == null) {
                current.left = new Node(value);
            } else {
                insertRec(current.left, value);
            }
        } else if (value > current.value) {
            if (current.right == null) {
                current.right = new Node(value);
            } else {
                insertRec(current.right, value);
            }
        }
    }
}"#,
        r#"public class AnotherTree {
    root = null;
    if (data != null) {
        root = new Node(data);
    }
    public void add(int data) {
        if (root == null) {
            root = new Node(data);
        } else {
            insertRecursive(root, data);
        }
    }

    private void insertRecursive(Node current, int data) {
        while (true) {
            if (data < current.value) {
                if (current.left == null) {
                    current.left = new Node(data);
                    break;
                } else {
                    current = current.left;
                }
            } else if (data > current.value) {
                if (current.right == null) {
                    current.right = new Node(data);
                    break;
                } else {
                    current = current.right;
                }
            }
        }
    }
}"#,
        r#"typedef struct Node {
    int value;
    struct Node* left;
    struct Node* right;
} Node;

void insert_node(Node** node, int value) {
    if (*node == NULL) {
        *node = malloc(sizeof(Node));
        (*node)->value = value;
        (*node)->left = NULL;
        (*node)->right = NULL;
    } else {
        insert_rec(*node, value);
    }
}

void insert_rec(Node* current, int value) {
    if (value < current->value) {
        if (current->left == NULL) {
            current->left = malloc(sizeof(Node));
            current->left->value = value;
            current->left->left = NULL;
            current->left->right = NULL;
        } else {
            insert_rec(current->left, value);
        }
    } else if (value > current->value) {
        if (current->right == NULL) {
            current->right = malloc(sizeof(Node));
            current->right->value = value;
            current->right->left = NULL;
            current->right->right = NULL;
        } else {
            insert_rec(current->right, value);
        }
    }
}"#,
        r#"class Node:
    def __init__(self, value):
        self.value = value
        self.left = None
        self.right = None

class BinaryTree:
    def __init__(self):
        self.root = None

    def insert(self, value):
        if self.root is None:
            self.root = Node(value)
        else:
            self.insert_rec(self.root, value)

    def insert_rec(self, current, value):
        if value < current.value:
            if current.left is None:
                current.left = Node(value)
            else:
                self.insert_rec(current.left, value)
        elif value > current.value:
            if current.right is None:
                current.right = Node(value)
            else:
                self.insert_rec(current.right, value)"#,
    ];
    #[test]
    fn parsing_works() {
        let res = process_code(INPUT.to_string()).expect("no errors");

        const LANGS: [Language; 4] = [
            Language::Java,
            Language::Java,
            Language::C,
            Language::Python,
        ];
        for (i, (&l, r)) in CODES.iter().zip(res.codes.into_iter()).enumerate() {
            assert_eq!((l.to_string(), LANGS[i]), r, "FAILED at index {i}");
        }

        assert_eq!(res.topic, "Binary Search Tree");

        let tree_init_plag = vec![
            PlagiarismEvent {
                file: 0,
                start: 2,
                end: 5,
            },
            PlagiarismEvent {
                file: 1,
                start: 2,
                end: 5,
            },
            PlagiarismEvent {
                file: 3,
                start: 3,
                end: 5,
            },
        ];

        let tree_insert_plag = vec![
            PlagiarismEvent {
                file: 0,
                start: 7,
                end: 11,
            },
            PlagiarismEvent {
                file: 1,
                start: 7,
                end: 11,
            },
            PlagiarismEvent {
                file: 2,
                start: 8,
                end: 15,
            },
            PlagiarismEvent {
                file: 3,
                start: 12,
                end: 15,
            },
        ];

        let map = HashMap::from([
            ("tree_init".to_string(), tree_init_plag),
            ("tree_insert".to_string(), tree_insert_plag),
        ]);

        assert_eq!(res.pairs, map);
    }
}
