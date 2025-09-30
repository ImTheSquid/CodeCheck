use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    time::Duration,
};

use ast::{Language, SyntaxTree, guess_language_from_extension};
use eyre::{Context, Ok, Result, bail};
use fancy_regex::Regex;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest};
use tokio::time::timeout;
use tokio_stream::StreamExt;

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
    pub topic: Option<String>,
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

    prompt = format!(
        "{prompt}\nYou have already explored these problems so don't pick them again: {}",
        banned_topics.join(",")
    );

    prompt
}

fn parse(code: String, lang: Language) -> Result<()> {
    // Add a NEWLINE to the end, just to be safe
    let code = format!("{code}\n");
    match lang {
        Language::C => {
            ast::c::CTree::try_from(code)?.symbol_tree()?;
        }
        Language::Cpp => {
            ast::cpp::CppTree::try_from(code)?.symbol_tree()?;
        }
        Language::Java => {
            ast::java::JavaTree::try_from(code)?.symbol_tree()?;
        }
        Language::Python => {
            ast::python::PythonTree::try_from(code)?.symbol_tree()?;
        }
    }

    Ok(())
}

fn process_code(
    mut response: String,
    disallow_non_plagiarized_code: bool,
) -> Result<GenerationOutput> {
    let topic_regex = Regex::new(r"<topic>(.+?)</topic>")?;
    let topic = topic_regex.captures(&response)?.map(|c| c[1].to_string());

    response = topic_regex.replace_all(&response, "").to_string();

    let code_regex = Regex::new(r#"(?ms)```([^\n]*)\n(.*?)\n?```"#)?;
    let open_plagiarism_regex = Regex::new(r#"\s*<plag .+?>"#)?;
    let plagiarism_regex = Regex::new(r#"(?ms)<plag (.+?)>(.*?)</plag( \1)?>"#)?;
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
            if open_plagiarism_regex.is_match(line)? {
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

            if line_buffer.is_empty() {
                line_buffer = line.to_string();
            } else {
                line_buffer = format!("{line_buffer}\n{line}");
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
                    bail!("No start when found full capture!");
                };
                let start = start + 1;
                // Exclusive
                let end = plag.lines().count() - 2 + start;
                if start >= end {
                    bail!("start not less than end! {start} >= {end}");
                }
                line_buffer = plagiarism_regex.replace(&line_buffer, "$2").to_string();
                found_open_plag_on_line_number = None;

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

            current_line_number += 1;
        }

        // Make sure code parses properly, otherwise it's useless
        parse(line_buffer.clone(), lang).context(format!(
            "Failed to parse generated {lang:?} code:\n=ORIGINAL=\n{code}\n===PLAG===\n{line_buffer}\n=========="
        ))?;

        cleaned_code.push((line_buffer, lang));
    }

    // First pass, filter self-plagiarism and remove unpaired identifiers
    plagiarism_by_id.retain(|_, v| {
        let mut seen = HashSet::new();
        v.retain(|val| {
            if seen.contains(&val.file) {
                return false;
            }
            seen.insert(val.file);
            true
        });
        v.len() >= 2
    });

    if plagiarism_by_id.is_empty() && disallow_non_plagiarized_code {
        bail!(
            "Invalid plagiarism dictionary! All data has been filtered out or removed in cleaning process."
        );
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
    disallow_non_plagiarized_code: bool,
) -> Result<GenerationOutput> {
    let mut response = ollama
        .generate_stream(GenerationRequest::new(
            model_name,
            generate_prompt(content_length, banned_topics),
        ))
        .await?;

    let mut response_vec = Vec::with_capacity(response.size_hint().0);
    while let Some(res) = timeout(Duration::from_secs(10), response.try_next())
        .await
        .context("timeout while resolving stream")?
        .context("stream encountered error")?
    {
        for resp in res {
            response_vec.push(resp.response);
        }
    }

    process_code(response_vec.join(""), disallow_non_plagiarized_code)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ast::Language;

    use crate::{PlagiarismEvent, process_code};

    const INPUT: &str = r#"<topic>Binary Search Tree</topic>
```java
public class BinaryTree {
    public void test() {
        <plag tree_init>//
        node = null;
        if (value != null) {
            node = new Node(value);
        }
        </plag tree_init>//
    }
    public void insert(int value) {
        <plag tree_insert>//
        if (node == null) {
            node = new Node(value);
        } else {
            insertRec(node, value);
        }
        </plag tree_insert>//
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
    public void test() {
        <plag tree_init>//
        root = null;
        if (data != null) {
            root = new Node(data);
        }
        </plag tree_init>//
    }
    public void add(int data) {
        <plag tree_insert>//
        if (root == null) {
            root = new Node(data);
        } else {
            insertRecursive(root, data);
        }
        </plag tree_insert>//
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
    <plag tree_insert>//
    if (*node == NULL) {
        *node = malloc(sizeof(Node));
        (*node)->value = value;
        (*node)->left = NULL;
        (*node)->right = NULL;
    } else {
        insert_rec(*node, value);
    }
    </plag tree_insert>//
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
        <plag tree_init>#
        self.value = value
        self.left = None
        self.right = None
        </plag tree_init>#

class BinaryTree:
    def __init__(self):
        self.root = None

    def insert(self, value):
        <plag tree_insert>#
        if self.root is None:
            self.root = Node(value)
        else:
            self.insert_rec(self.root, value)
        </plag tree_insert>#

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
    public void test() {
        //
        node = null;
        if (value != null) {
            node = new Node(value);
        }
        //
    }
    public void insert(int value) {
        //
        if (node == null) {
            node = new Node(value);
        } else {
            insertRec(node, value);
        }
        //
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
    public void test() {
        //
        root = null;
        if (data != null) {
            root = new Node(data);
        }
        //
    }
    public void add(int data) {
        //
        if (root == null) {
            root = new Node(data);
        } else {
            insertRecursive(root, data);
        }
        //
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
    //
    if (*node == NULL) {
        *node = malloc(sizeof(Node));
        (*node)->value = value;
        (*node)->left = NULL;
        (*node)->right = NULL;
    } else {
        insert_rec(*node, value);
    }
    //
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
        #
        self.value = value
        self.left = None
        self.right = None
        #

class BinaryTree:
    def __init__(self):
        self.root = None

    def insert(self, value):
        #
        if self.root is None:
            self.root = Node(value)
        else:
            self.insert_rec(self.root, value)
        #

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
        let res = process_code(INPUT.to_string(), true).expect("no errors");

        const LANGS: [Language; 4] = [
            Language::Java,
            Language::Java,
            Language::C,
            Language::Python,
        ];
        for (i, (&l, r)) in CODES.iter().zip(res.codes.into_iter()).enumerate() {
            if (l.to_string(), LANGS[i]) != r {
                println!("LEFT: {l}");
                println!("RIGHT: {}", r.0);
                panic!("FAILED at index {i}");
            }
        }

        assert_eq!(res.topic, Some("Binary Search Tree".to_string()));

        let tree_init_plag = vec![
            PlagiarismEvent {
                file: 0,
                start: 4,
                end: 8,
            },
            PlagiarismEvent {
                file: 1,
                start: 4,
                end: 8,
            },
            PlagiarismEvent {
                file: 3,
                start: 4,
                end: 7,
            },
        ];

        let tree_insert_plag = vec![
            PlagiarismEvent {
                file: 0,
                start: 12,
                end: 17,
            },
            PlagiarismEvent {
                file: 1,
                start: 12,
                end: 17,
            },
            PlagiarismEvent {
                file: 2,
                start: 9,
                end: 17,
            },
            PlagiarismEvent {
                file: 3,
                start: 15,
                end: 19,
            },
        ];

        let map = HashMap::from([
            ("tree_init".to_string(), tree_init_plag),
            ("tree_insert".to_string(), tree_insert_plag),
        ]);

        assert_eq!(res.pairs, map);
    }

    const HARD_PARSE: &str = "<topic>test</topic>
```java
public class Main {
    public static int longestIncreasingSubsequence(int[] sequence) {
        int[] lengths = new int[sequence.length];
        for (int i = 0; i < sequence.length; i++) {
            lengths[i] = 1;
        }
<plag lis3>
        int[] indices = new int[sequence.length];
        int indexCount = 0;
        for (int i = 0; i < sequence.length; i++) {
            if (lengths[i] == max(lengths)) {
                indices[indexCount++] = i;
            }
        }
</plag lis3>
<plag lis1>
        for (int i = 1; i < sequence.length; i++) {
            for (int j = 0; j < i; j++) {
                if (sequence[i] > sequence[j]) {
                    lengths[i] = Math.max(lengths[i], lengths[j] + 1);
                }
            }
        }
</plag lis1>
        int max = lengths[0];
        for (int i = 1; i < lengths.length; i++) {
            if (lengths[i] > max) {
                max = lengths[i];
            }
        }
        return max;
    }

    public static void main(String[] args) {
        int[] sequence = {10, 22, 9, 33, 21, 50, 41, 60};
        System.out.println(longestIncreasingSubsequence(sequence));
    }
}
```

```java
public class Main {
    public static int longestIncreasingSubsequence(int[] sequence) {
        int[] lengths = new int[sequence.length];
        for (int i = 0; i < sequence.length; i++) {
            lengths[i] = 1;
        }
<plag lis3>
        int[] indices = new int[sequence.length];
        int indexCount = 0;
        for (int i = 0; i < sequence.length; i++) {
            if (lengths[i] == max(lengths)) {
                indices[indexCount++] = i;
            }
        }
</plag lis3>
<plag lis1>
        for (int i = 1; i < sequence.length; i++) {
            for (int j = 0; j < i; j++) {
                if (sequence[i] > sequence[j]) {
                    lengths[i] = Math.max(lengths[i], lengths[j] + 1);
                }
            }
        }
</plag lis1>
        int max = lengths[0];
        for (int i = 1; i < lengths.length; i++) {
            if (lengths[i] > max) {
                max = lengths[i];
            }
        }
        return max;
    }

    public static void main(String[] args) {
        int[] sequence = {10, 22, 9, 33, 21, 50, 41, 60};
        System.out.println(longestIncreasingSubsequence(sequence));
    }
}
```
```python
# Plagiarized Implementation 1
def knapsack(capacity, weights, values):
    n = len(values)
    dp = [[0 for _ in range(capacity + 1)] for _ in range(n + 1)]
    <plag knapsack_init>
    for i in range(1, n + 1):
        for j in range(1, capacity + 1):
            if weights[i - 1] <= j:
                dp[i][j] = max(values[i - 1] + dp[i - 1][j - weights[i - 1]], dp[i - 1][j])
            else:
                dp[i][j] = dp[i - 1][j]
                </plag knapsack_init>
    return dp[n][capacity]

print(knapsack(10, [3, 4, 5], [60, 100, 120]))
```
```python
# Plagiarized Implementation 1
def knapsack(capacity, weights, values):
    n = len(values)
    dp = [[0 for _ in range(capacity + 1)] for _ in range(n + 1)]
    <plag knapsack_init>
    for i in range(1, n + 1):
        for j in range(1, capacity + 1):
            if weights[i - 1] <= j:
                dp[i][j] = max(values[i - 1] + dp[i - 1][j - weights[i - 1]], dp[i - 1][j])
            else:
                dp[i][j] = dp[i - 1][j]
                </plag knapsack_init>
    return dp[n][capacity]

print(knapsack(10, [3, 4, 5], [60, 100, 120]))
```";

    #[test]
    fn test_hard_parse() {
        process_code(HARD_PARSE.to_string(), true).expect("good");
    }
}
